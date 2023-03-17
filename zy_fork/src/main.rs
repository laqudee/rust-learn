use std::env;
use std::ffi::OsStr;
use std::io;
use std::path::{Component, Path, PathBuf};
use std::process;
use std::sync::Arc;
use std::time;

use actix_web::http::{header, StatusCode};
use actix_web::{guard, web, App, HttpServer};
use actix_web::{HttpRequest, HttpResponse};
use clap::Parser;
use color_eyre::eyre::Result;
use humantime::format_duration;
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;

mod cli;
mod exit;
mod middleware;
#[macro_use]
mod macros;

// /a => a
// a/../b => b
// /a/b/../c/./d => a/c/d
// C:\a => Error
fn normalize_path<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    let mut buf = PathBuf::new();
    for c in path.as_ref().components() {
        match c {
            Component::Normal(c) => buf.push(c),
            Component::ParentDir => {
                buf.pop();
            }
            Component::CurDir | Component::RootDir => {}
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "prefix is not supported",
                ))
            }
        }
    }
    for c in buf.components() {
        assert!(matches!(c, Component::Normal(_)));
    }
    Ok(buf)
}

#[derive(Copy, Clone)]
enum CachePolicy {
    NoCache,
    ShouldCache,
    Indeterminate,
}

#[derive(Copy, Clone)]
enum PathSource {
    Client,
    Server,
}

fn serve(
    req: &HttpRequest,
    path: &str,
    source: PathSource,
    state: &ServerState,
) -> Option<HttpResponse> {
    let path = normalize_path(Path::new(path)).ok()?;

    if let PathSource::Client = source {
        if !state.args.all
            && path
                .file_name()
                .map_or(false, |name| name.to_string_lossy().starts_with('.'))
        {
            return None;
        }
    }

    let mut path = state
        .args
        .dir
        .canonical
        .join(if path.as_os_str().is_empty() {
            state.args.dir.canonical.join(&state.args.index)
        } else {
            state.args.dir.canonical.join(path).canonicalize().ok()?
        });

    if let PathSource::Client = source {
        if !path.starts_with(&state.args.dir.canonical) && !state.args.follow_links {
            return None;
        }
    }

    if path.is_dir() {
        path = path.join("index.html").canonicalize().ok()?;
    }

    if !path.is_file() {
        return None;
    }

    if state.args.verbose {
        debug!(target: "zy::serve", path=%path.strip_prefix(&state.args.dir.canonical).ok()?.display());
    }

    let file = actix_files::NamedFile::open(&path).ok()?;

    let mut res = file
        .use_etag(true)
        .prefer_utf8(true)
        .use_last_modified(true)
        .disable_content_disposition()
        .into_response(req);

    if let StatusCode::OK | StatusCode::PARTIAL_CONTENT = res.status() {
        let ext = path.extension().and_then(OsStr::to_str);
        let mime = ext.map_or(mime::APPLICATION_OCTET_STREAM, |ext| {
            actix_files::file_extension_to_mime(ext)
        });

        let cache_policy = match (mime.type_(), mime.subtype()) {
            (mime::TEXT, mime::HTML) => CachePolicy::NoCache,
            (mime::APPLICATION, mime::JAVASCRIPT)
            | (mime::TEXT, mime::CSS)
            | (mime::IMAGE, _)
            | (mime::TEXT, _)
            | (mime::FONT, _) => CachePolicy::ShouldCache,
            _ if matches!(ext, Some("otf") | Some("woff")) => CachePolicy::ShouldCache,
            _ => CachePolicy::Indeterminate,
        };

        let mut cache_directives = vec![];

        match cache_policy {
            CachePolicy::NoCache => {
                cache_directives.extend([
                    header::CacheDirective::NoCache,
                    header::CacheDirective::NoStore,
                ]);
            }
            CachePolicy::ShouldCache => {
                cache_directives.extend([
                    header::CacheDirective::Public,
                    header::CacheDirective::MaxAge(state.args.cache),
                ]);
            }
            CachePolicy::Indeterminate => {}
        }

        if let Ok((k, v)) =
            header::TryIntoHeaderPair::try_into_pair(header::CacheControl(cache_directives))
        {
            res.headers_mut().insert(k, v);
        }
    }

    Some(res)
}

async fn index(
    req: HttpRequest,
    path: web::Path<String>,
    state: web::Data<Arc<ServerState>>,
) -> HttpResponse {
    if state.args.verbose {
        debug!(
            target: "zy::request",
            version = ?req.version(),
            method = %req.method(),
            uri = %req.uri(),
        );
    }

    let mut res = match serve(&req, &path, PathSource::Client, &state) {
        Some(res) => res,
        None => {
            if state.args.spa {
                let accepts_html = <header::Accept as header::Header>::parse(&req)
                    .map_or(false, |accept| {
                        accept.iter().any(|mime| mime.item == "text/html")
                    });
                if accepts_html {
                    if state.args.verbose {
                        info!(target: "zy::serve", "SPA routing to {}", state.args.index);
                    }
                    if let Some(res) = serve(&req, &state.args.index, PathSource::Server, &state) {
                        return res;
                    }
                }
            }
            if state.args.verbose {
                info!(target: "zy::serve", "not found, serving {}", state.args.not_found);
            }
            match serve(&req, &state.args.not_found, PathSource::Server, &state) {
                Some(mut resp) => {
                    *resp.status_mut() = StatusCode::NOT_FOUND;
                    resp
                }
                None => {
                    if state.args.verbose {
                        info!(target: "zy::serve", "{} not found, omitting response body", state.args.not_found);
                    }
                    HttpResponse::build(StatusCode::NOT_FOUND).finish()
                }
            }
        }
    };

    if !state.args.no_cors {
        res.headers_mut().insert(
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            header::HeaderValue::from_static("*"),
        );
    }

    res
}

pub struct ServerState {
    args: cli::Args,
}

async fn init_app() -> Result<()> {
    let mut args = cli::Args::parse();

    if let Ok(port) = env::var("PORT") {
        if port.parse::<u16>().is_err() {
            eprintln!(
                "warning: invalid PORT environment variable: {:?}, ignoring",
                port
            );
        }
    }

    if args.listen.is_empty() {
        args.listen.push(cli::addr_from_str("127.0.0.1").unwrap());
    }

    print_block! {
        "Process ID" => process::id()
        "Base Directory" => if args.dir.is_current_dir() {
            format!("{} (current directory)", args.dir.raw.display())
        } else {
            args.dir.raw.display().to_string()
        }
        "Cache Time" => format_duration(time::Duration::from_secs(args.cache as u64))
        "Single Page App?" => args.spa
        "Index File" => args.index
        "404 Fallback" => args.not_found
        "Cross-Origin Resource Sharing" => if args.no_cors {
            "disabled"
        } else {
            "(any domain)"
        }
        "Anonymize Server" => args.anonymize
        "Will Serve Hidden Files" => args.all
        "Follow Symlinks Outside Root" => args.follow_links
        "Confirm Exit" => args.confirm_exit
        "Be Verbose" => args.verbose
    }

    let server_state = Arc::new(ServerState { args });

    let mut server = {
        let server_state = server_state.clone();
        HttpServer::new(move || {
            App::new()
                .wrap(middleware::ZyServer {
                    anonymize: server_state.args.anonymize,
                })
                .app_data(web::Data::new(server_state.clone()))
                .service(
                    web::resource("/{path:.*}")
                        .guard(guard::Any(guard::Get()).or(guard::Head()))
                        .wrap(middleware::Compress::default())
                        .to(index),
                )
        })
        .disable_signals()
    };

    for addr in &server_state.args.listen {
        server = server.bind(addr)?;
        println!("Listening on http://{}", addr);
    }

    let server = server.run();

    let server_handle = server.handle();

    tokio::select! {
        _ = server => {}
        _ = exit::on_signal(
            server_state.args.confirm_exit,
            |graceful| async move {
                if graceful {
                    info!("Starting graceful shutdown");
                } else {
                    info!("Shutting down immediately");
                }
                server_handle.stop(graceful).await;
            }
        ) => {}
    }

    Ok(())
}

fn setup() -> Result<()> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1");
    }
    color_eyre::install()?;

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("zy=info".parse()?)
                .add_directive("zy=debug".parse()?),
        )
        .init();

    Ok(())
}

#[actix_web::main]
async fn main() -> Result<()> {
    setup()?;

    init_app().await?;

    Ok(())
}
