use http_body_util::{BodyExt, Full};
use hyper::header::{self, ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN};
use hyper::{body::Bytes, Response, StatusCode};
use serde::Serialize;

pub type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

pub fn full<T: Into<Bytes>>(chunk: T) -> BoxBody 
{
    Full::new(chunk.into())
    .map_err(|never| match never {})
    .boxed()
}

pub fn to_body(bytes: Bytes) -> BoxBody
{
    Full::new(bytes)
        .map_err(|never| match never {})
    .boxed()
}  

pub fn error_response(err: String, code: StatusCode) -> Response<BoxBody>
{
    Response::builder()
    .status(code)
    .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
    .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
    .header(ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
    .header(ACCESS_CONTROL_ALLOW_METHODS, "GET, PUSH")
    .body(to_body(Bytes::from(err))).unwrap()
}
pub fn error_empty_response(code: StatusCode) -> Response<BoxBody>
{
    Response::builder()
    .status(code)
    .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
.header(ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
    .header(ACCESS_CONTROL_ALLOW_METHODS, "GET, PUSH")
.header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
    .body(to_body(Bytes::new())).unwrap()
}
pub fn ok_response(msg: String) -> Response<BoxBody>
{
    Response::builder()
    .status(StatusCode::OK)
    .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
    .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
    .header(ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
    .header(ACCESS_CONTROL_ALLOW_METHODS, "GET, PUSH")
    .body(to_body(Bytes::from(msg))).unwrap()
}
pub fn json_response<S: Serialize>(obj: &S) -> Response<BoxBody>
{
    let result = serde_json::to_string(obj).unwrap();
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json ")
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
    .header(ACCESS_CONTROL_ALLOW_METHODS, "GET, PUSH")
        .body(to_body(Bytes::from(result))).unwrap()
}

pub fn unauthorized_response() -> Response<BoxBody>
{
    Response::builder()
    .status(StatusCode::UNAUTHORIZED)
    .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
    .header(ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
    .header(ACCESS_CONTROL_ALLOW_METHODS, "GET, PUSH")
    .body(to_body(Bytes::from_static(b"Unauthorized")))
    .unwrap()
}