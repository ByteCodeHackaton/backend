use http_body_util::Full;
use hyper::{body::{Body, Bytes, Incoming}, header::{self, HeaderValue, ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, AUTHORIZATION, CONTENT_TYPE, ORIGIN}, Method, Request, Response};
use tower::{ServiceBuilder, ServiceExt, Service};
use tower_http::cors::{CorsLayer, Any};
use std::convert::Infallible;
pub type BoxBody = http_body_util::combinators::BoxBody<(), hyper::Error>;

async fn handle(request: Request<Full<Bytes>>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::default()))
}

pub fn cors_layer() -> CorsLayer
{
    let cors_layer = CorsLayer::new()
            .allow_origin([
                "http://localhost:5173".parse::<HeaderValue>().unwrap(),
                "http://localhost".parse::<HeaderValue>().unwrap(),
                "http://213.159.215.231".parse::<HeaderValue>().unwrap()
            ])
            .allow_methods([Method::GET, Method::POST, Method::OPTIONS, Method::PUT, Method::HEAD, Method::DELETE])
            .allow_headers([ORIGIN, ACCEPT, CONTENT_TYPE, ACCESS_CONTROL_ALLOW_HEADERS, AUTHORIZATION])
            .allow_credentials(true);
        //"Access-Control-Allow-Headers", "Access-Control-Allow-Headers, Origin,Accept, X-Requested-With, Content-Type, Access-Control-Request-Method, Access-Control-Request-Headers"
            //.allow_headers(vec![AUTHORIZATION, ACCEPT]);
    return cors_layer;
}

#[test]
fn tett()
{
   let ttt = "localhost".parse::<HeaderValue>().unwrap();
}
// assert_eq!(
//     response.headers().get(header::ACCESS_CONTROL_ALLOW_ORIGIN).unwrap(),
//     "*",