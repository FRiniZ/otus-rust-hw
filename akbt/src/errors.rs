use thiserror::Error;

use crate::gzip::GzMsg;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("rdkafka error")]
    Kafka(#[from] rdkafka::error::KafkaError),
    #[error("Topic:{0} not found")]
    TopicNotFound(String),
    #[error("IoError")]
    IoError(#[from] std::io::Error),
    #[error("Be careful the file: `{0}` is exists")]
    FileExists(String),
    #[error("Can't find file: `{0}`")]
    FileNotExists(String),
    #[error("Can't send message to encoder")]
    SendError(#[from] std::sync::mpsc::SendError<GzMsg>),
}
