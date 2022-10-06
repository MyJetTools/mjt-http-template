use crate::{FromBytes, AsBytes};

pub static TEST_EVENT_SB_TOPIC_NAME: &'static str = "test-event";

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestEventSbContract {
    #[prost(string, tag = "1")]
    pub test: String
}

impl AsBytes for TestEventSbContract {
    fn as_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        prost::Message::encode(self, &mut result).unwrap();
        result
    }
}

impl FromBytes for TestEventSbContract {
    fn from_bytes(src: &[u8]) -> Self {
        prost::Message::decode(src).unwrap()
    }
}
