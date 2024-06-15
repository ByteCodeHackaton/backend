use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrderError 
{
    #[error(transparent)]
    DeserializeError(#[from] serde_json::Error),
    #[error(transparent)]
    HyperError(#[from] hyper::Error),
    #[error(transparent)]
    HyperHttpError(#[from] hyper::http::Error),
    #[error(transparent)]
    HyperLegasyClientError(#[from] hyper_util::client::legacy::Error),
    #[error("По данным параметрам заявки `{0}`")]
    NotFreeWorkers(String),
    #[error("Ошибка сервиса станций `{0}`")]
    StationServiceError(String),
    #[error(transparent)]
    RequwestError(#[from] reqwest::Error),
    #[error("Ошибка подключения к сервису `{0}` при отправке сообщения")]
    SendError(String),
}