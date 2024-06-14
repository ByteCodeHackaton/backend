use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrderError 
{
    #[error("По данным параметрам заявки `{0}`")]
    NotFreeWorkers(String),
    #[error("Ошибка сервиса станций `{0}`")]

    StationServiceError(String),
    #[error(transparent)]
    RequwestError(#[from] reqwest::Error),
    #[error("Ошибка подключения к сервису `{0}` при отправке сообщения")]
    SendError(String),
}