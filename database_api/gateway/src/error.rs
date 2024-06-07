use thiserror::Error;

#[derive(Error, Debug)]
pub enum GatewayError 
{
    // #[error("По данным параметрам заявки `{0}`")]
    // NotFreeWorkers(String),
    // #[error("Ошибка сервиса станций `{0}`")]

    // StationServiceError(String),
    #[error(transparent)]
    HyperError(#[from] hyper::Error),
    #[error(transparent)]
    HyperHttpError(#[from] hyper::http::Error),
    #[error(transparent)]
    HyperLegasyClientError(#[from] hyper_util::client::legacy::Error),
    #[error(transparent)]
    DeserializeError(#[from] serde_json::Error),
    #[error("Ошибка подключения к сервису `{0}` при отправке сообщения")]
    SendError(String),
}

// impl std::fmt::Display for GatewayError
// {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
//   {
//     match self 
//     {
//       GatewayError::HyperError(e) => f.write_str(&e.to_string()),
//     }
//   }
// }

impl serde::Serialize for GatewayError
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
  S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}