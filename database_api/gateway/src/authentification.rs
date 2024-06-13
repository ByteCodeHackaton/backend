use crate::jwt::Claims;
use crate::{body_helpers::*, GatewayError};
use crate::error_response;
use http_body_util::BodyExt;
use hyper::{body::Incoming, Request, Response, StatusCode};
use logger::backtrace;
use super::jwt::KEY;
use serde::{Deserialize, Serialize};

pub async fn is_autentificated(req: &Request<Incoming>) -> bool
{
    match req.headers().get("Authorization") 
    {
        Some(value) => 
        {
            let token_str = value.to_str().unwrap_or("").replace("Bearer ", "");
            logger::info!("Проверка токена->{}", token_str);
            let key = KEY.lock().await;

            let v = key.validate_access(&token_str);
            if let Ok(_) = v
            {
                true
            }
            else 
            {
                let e = v.err().unwrap().to_string();
                logger::error!("{} {}", &e, backtrace!());
                false
            }
        },
        None => 
        {
            let e = "Отсуствует заголовок Authorization!";
            logger::error!("{}", e);
            false
        }
    }
}
pub async fn get_claims(req: &Request<Incoming>) -> Option<Claims>
{
    match req.headers().get("Authorization") 
    {
        Some(value) => 
        {
            let token_str = value.to_str().unwrap_or("").replace("Bearer ", "");
            logger::info!("Проверка токена->{}", token_str);
            let key = KEY.lock().await;

            let v = key.validate_access(&token_str);
            if let Ok(cl) = v
            {
                Some(cl.claims)
            }
            else 
            {
                let e = v.err().unwrap().to_string();
                logger::error!("{} {}", &e, backtrace!());
                None
            }
        },
        None => 
        {
            let e = "Отсуствует заголовок Authorization";
            logger::error!("{}", e);
            None
        }
    }
}
pub async fn verify_token(req: &Request<Incoming>) -> Result<Response<BoxBody>, GatewayError> 
{
    let is_auth = is_autentificated(&req).await;
    if is_auth
    {
        return Ok(ok_response("Вы аутентифицированы".to_owned()));
    }
    else 
    {
        return Ok(unauthorized_response());
        
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTokens
{
    pub access: String,
    pub refresh: String
}
pub async fn update_tokens(req: Request<Incoming>) -> Result<Response<BoxBody>, GatewayError> 
{
    let body = req.collect().await?.to_bytes();
    let tokens: Result<UpdateTokens, serde_json::Error> = serde_json::from_slice(&body);
   
    if tokens.is_err()
    {
        logger::error!("Неверный формат для обновления токенов -> {}", tokens.err().unwrap());
        let resp = error_response("Неверный формат для обновления токенов, необходим формат: '{ \"access\": string, \"refresh\": string}".to_owned(), StatusCode::BAD_REQUEST);
        return  Ok(resp);    
    }
    let tokens = tokens.unwrap();
    let mut keys = KEY.lock().await;
    let res = keys.update_keys(&tokens.refresh);
    if res.is_err()
    {
        return Ok(error_response(res.err().unwrap().to_string(), StatusCode::NOT_ACCEPTABLE));
    }
    let updated = res.unwrap();
    let update_tokens = UpdateTokens
    {
        access: updated.1,
        refresh: updated.0
    };
    let resp = json_response(&update_tokens);
    return  Ok(resp);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crendentials
{
    login: String,
    password: String
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationResponse
{
    user_id: String,
    role: String,
    refresh_key: String,
    access_key: String
}
//пока самое сомнительное решение запрашивать аутентификацию у строннего сервиса
//позже неужно добавить адрес эндпоинта для аутентификации в аргументы при старте сервиса
pub async fn authentificate(req: Request<Incoming>) -> Result<Response<BoxBody>, GatewayError> 
{
    
    let body = req.collect().await?.to_bytes();
    let crendentials: Result<Crendentials, serde_json::Error> = serde_json::from_slice(&body);
    if crendentials.is_err()
    {
        let str = String::from_utf8_lossy(&body);
        logger::error!("Неверный формат для авторизации ({}) -> {}", str, crendentials.err().unwrap());
        let resp = error_response(["Неверный формат для авторизации", str.as_ref(), ", необходим формат: '{ \"login\": string, \"password\": string}"].concat(), StatusCode::BAD_REQUEST);
        return  Ok(resp);    
    }
    let crendentials = crendentials.unwrap();
    //тут будет проверка этого дела, а пока эмуляция типа мы получили id юзера после авторизации
    let user_id = &crendentials.login;
    let mut keys = KEY.lock().await;
    let res = keys.get_pair(user_id);
    let authorized = AuthorizationResponse
    {
        user_id: user_id.to_owned(),
        role: "Оператор".to_owned(),
        refresh_key: res.0,
        access_key: res.1
    };
    let resp = json_response(&authorized);
    return  Ok(resp);
}
