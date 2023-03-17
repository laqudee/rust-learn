use std::future::{ready, Future, Ready};
use std::pin::Pin;
use std::task::{Context, Poll};

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header;
pub use actix_web::middleware::Compress;

pub struct ZyServer {
    pub anonymize: bool,
}

impl<S, B> Transform<S, ServiceRequest> for ZyServer
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>>,
    S::Future: 'static,
    B: 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Transform = ZyServerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ZyServerMiddleware {
            service,
            anonymize: self.anonymize,
        }))
    }
}

pub struct ZyServerMiddleware<S> {
    service: S,
    anonymize: bool,
}

impl<S, B> Service<ServiceRequest> for ZyServerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>>,
    S::Future: 'static,
    B: 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);

        let anonymous = self.anonymize;
        Box::pin(async move {
            let mut res = fut.await?;
            if !anonymous {
                res.headers_mut().insert(
                    header::SERVER,
                    header::HeaderValue::from_static(concat!("Zy/", env!("CARGO_PKG_VERSION"))),
                );

                res.headers_mut().insert(
                    "X-Powered-By"
                        .try_into()
                        .expect("x-powered-by should be valid header"),
                    header::HeaderValue::from_static(concat!("Zy/", env!("CARGO_PKG_VERSION"))),
                );
            }

            Ok(res)
        })
    }
}
