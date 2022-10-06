use std::sync::Arc;

use my_service_bus_tcp_client::subscribers::{MessagesReader, SubscriberCallback};

use crate::{AppContext, TestEventSbContract, read_my_sb_message};

pub struct TestEventListener {
    pub app: Arc<AppContext>,
}

impl TestEventListener {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

#[async_trait::async_trait]
impl SubscriberCallback for TestEventListener {
    async fn new_events(&self, mut messages_reader: MessagesReader) {
        for message in messages_reader.get_messages() {
            let result =
                read_my_sb_message::<TestEventSbContract>(&message);

            if let Err(err) = &result {
                println!("Can not read message. Err: {:?}", err);
                messages_reader.handled_ok(&message);
                return;
            }

            {% if is_use_telemetry %}let (test_event, telemetry) = result.unwrap();{% else%} let test_event = result.unwrap();{% endif %}

            println!("event: {:?}", test_event);

        }
    }
}
