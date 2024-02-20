#![allow(dead_code)]
pub mod kafka_messages {
    use prost::Message;

    include!(concat!(env!("OUT_DIR"), "/kafka_messages.rs"));
    pub fn kafka_message_new(
        key: Option<Vec<u8>>,
        value: Option<Vec<u8>>,
        partition: Option<u32>,
        headers: Vec<Vec<u8>>,
    ) -> KafkaMessage {
        KafkaMessage {
            key,
            value,
            partition,
            headers,
        }
    }

    pub fn kafka_message_pack(msg: &KafkaMessage) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.reserve(msg.encoded_len());
        msg.encode(&mut buf).unwrap();
        buf
    }

    pub fn kafka_message_unpack(buf: &[u8]) -> Result<KafkaMessage, String> {
        let res = KafkaMessage::decode(buf);
        if res.is_err() {
            return Err(format!("Can't decode message:{}", res.unwrap_err()));
        }
        Ok(res.unwrap())
    }

    pub fn kafka_message_len(msg: &KafkaMessage) -> usize {
        msg.encoded_len()
    }
}
