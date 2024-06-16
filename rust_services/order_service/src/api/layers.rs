use axum::{response::{Response}, http::HeaderValue};//http::{Request, Response, Method, header};
use hyper::{Body, Method, Request, header::{self, AUTHORIZATION, ACCEPT, ORIGIN, CONTENT_TYPE, ACCESS_CONTROL_ALLOW_HEADERS}};
use tower::{ServiceBuilder, ServiceExt, Service};
use tower_http::cors::{CorsLayer, Any};
use std::convert::Infallible;

async fn handle(request: Request<Body>) -> Result<Response<Body>, Infallible> 
{
    Ok(Response::new(Body::empty()))
}

async fn cors()
{
    let cors = CorsLayer::new()
    .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
    .allow_origin(Any)
    .allow_credentials(false);

    let mut service = ServiceBuilder::new()
        .layer(cors)
        .service_fn(handle);

    let request = Request::builder()
        .header(header::ORIGIN, "localhost")
        .body(Body::empty())
        .unwrap();

    let response = service
        .ready()
        .await;
    if response.is_ok()
    {
        response.unwrap().call(request)
        .await;
    }
}

pub fn cors_layer() -> CorsLayer
{
    let cors_layer = CorsLayer::new()
            .allow_origin("*".parse::<HeaderValue>().unwrap())
            .allow_methods([Method::GET, Method::POST, Method::OPTIONS, Method::PUT, Method::HEAD])
            .allow_headers([ORIGIN, ACCEPT, CONTENT_TYPE, ACCESS_CONTROL_ALLOW_HEADERS]);
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