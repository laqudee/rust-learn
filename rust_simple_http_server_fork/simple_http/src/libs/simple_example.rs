use http::{
    http_method::method::Method,
    http_request::request::HttpRequest,
    http_response::response::HttpResponse,
    http_router::{router::Router, router_handler::RouterHandler},
    http_server::server::HttpServer,
};

use std::fs;
use std::path::Path;

pub fn play() {
    // 初始化一个Application
    let mut server = HttpServer::application();
    // 创建一个Router
    let mut router = Router::new();
    // 将route函数添加到router中
    router.add_route(RouterHandler::new(Method::GET, "/", |_r, w| {
        w.write_str("Hello Rust World");
    }));
    router.add_route(RouterHandler::new(Method::GET, "/hi", router_fn));

    // 测试传输html文件到前台渲染
    router.add_route(RouterHandler::new(Method::GET, "/hello", router_html_fn));

    // 设置Http监听地址，并挂载Router到该服务中
    server
        .configure(HttpServer::set_addr("127.0.0.1:8081"))
        .mount_route(router);
    //启动服务器
    server.start();
    print!("{:?}", server);
}

fn router_fn(_r: &HttpRequest, w: &mut HttpResponse) {
    // 输出中文需要添加响应头，否则会乱码
    w.insert_header("Content-Type", "text/html;charset=utf-8");
    println!("remote addr: {}", _r.get_remote_addr());
    if let Some(header) = _r.get_header("User-Agent") {
        println!("User-Agent: {}", header);
    }
    w.write_str("你好Rust世界！");
}

fn router_html_fn(_r: &HttpRequest, w:&mut HttpResponse) {
    w.insert_header("Content-Type", "text/html;charset=utf-8");
    println!("remote html addr: {}", _r.get_remote_addr());
    if let Some(header) = _r.get_header("User-Agent") {
        println!("User-Agent: {} ", header);
    }

    let file_path = Path::new("hello.html");
    let contents = fs::read_to_string(file_path).unwrap();
    w.write_str(&contents);
}

