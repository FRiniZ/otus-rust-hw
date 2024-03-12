use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AppError {
    #[error("rdkafka error")]
    Kafka(#[from] rdkafka::error::KafkaError),
    #[error("Topic:{0} not found")]
    TopicNotFound(String),
    #[error("IoError: {0}")]
    IoError(String),
    #[error("Be careful the file: `{0}` is exists")]
    FileExists(String),
    #[error("Can't find file: `{0}`")]
    FileNotExists(String),
    #[error("Can't send message to encoder:{0}")]
    Send2Encoder(String),
    #[error("EOF")]
    EOF,
}
