use axum::{response::{IntoResponse, Response}, http::StatusCode, Json};
use serde_json::json;

pub enum AppError 
{
    Error(String)
}


impl IntoResponse for AppError 
{
    fn into_response(self) -> Response 
    {
        let (status, error_message) = match self {
            AppError::Error(e) =>
            {
                (StatusCode::NOT_FOUND, e)
            }
        };
        //let body = Json((crate::models::Response::<String>::err(&error_message)));
        (status, super::response::from_err(error_message).as_json()).into_response()
    }
}