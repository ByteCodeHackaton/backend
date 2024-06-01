use axum::{response::IntoResponse, Json};
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response<T: serde::Serialize + Clone + Send>
{
    success: bool,
    message: String,
    response_object: Option<T>
}
impl<T: serde::Serialize + Clone + Send> Response<T>
{
    pub fn new(obj: T) -> Json<Response<T>>
    {
        Json(Response
        {
            success: true,
            message: "".to_owned(),
            response_object: Some(obj)
        })
    }
    pub fn as_json(&self) -> String
    {
        serde_json::to_string(self).unwrap()
    }
    pub fn as_response(&self) -> Json<Response<T>>
    {
        Json(self.clone())
    }
    pub fn from_err(e: String) -> Json<Response<T>>
    {
        Json(Response
        {
            success: false,
            message: e,
            response_object: None
        })
    }
}

pub fn from_err(e: String) -> Response<bool>
{
    Response
    {
        success: false,
        message: e,
        response_object: None
    }
}