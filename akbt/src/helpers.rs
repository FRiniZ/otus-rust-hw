pub struct Bytes(Option<Vec<u8>>);

impl From<Option<&[u8]>> for Bytes {
    fn from(value: Option<&[u8]>) -> Self {
        match value {
            Some(v) => Bytes(Some(v.to_vec())),
            None => Bytes(None),
        }
    }
}

impl Into<Option<Vec<u8>>> for Bytes {
    fn into(self) -> Option<Vec<u8>> {
        self.0
    }
}
