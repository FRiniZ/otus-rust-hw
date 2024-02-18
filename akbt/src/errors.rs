use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("rdkafka error")]
    Kafka(#[from] rdkafka::error::KafkaError),
    #[error("Topic:{} not found", 0)]
    TopicNotFound(String),
    #[error("Can't write to file")]
    IoError(#[from] std::io::Error),
}
