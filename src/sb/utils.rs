use my_service_bus_tcp_client::subscribers::MySbDeliveredMessage;
{% if is_use_telemetry %}use my_telemetry::MyTelemetryContext;{% endif %}
use prost::{DecodeError};
{% if is_use_telemetry %}use rust_extensions::date_time::DateTimeAsMicroseconds;

pub const MY_TELEMETRY_HEADER: &str = "process-id";{% endif %}

pub trait AsBytes {
    fn as_bytes(&self) -> Vec<u8>;
}

pub trait FromBytes {
    fn from_bytes(src: &[u8]) -> Self;
}
{% if is_use_telemetry %}
pub fn read_my_sb_message<TMessage: Send + Sync + Default + FromBytes>(
    msg: &MySbDeliveredMessage,
) -> Result<(TMessage, MyTelemetryContext), DecodeError> {
    let result: TMessage = TMessage::from_bytes(msg.content.as_slice());

    if msg.headers.is_none() {
        let result = (
            result,
            MyTelemetryContext {
                process_id: DateTimeAsMicroseconds::now().unix_microseconds,
            },
        );
        return Ok(result);
    }

    let headers = msg.headers.as_ref().unwrap();

    if let Some(process_id) = headers.get(MY_TELEMETRY_HEADER) {
        let process_id = process_id.parse::<i64>().unwrap();
        return Ok((result, MyTelemetryContext { process_id }));
    }

    let result = (
        result,
        MyTelemetryContext {
            process_id: DateTimeAsMicroseconds::now().unix_microseconds,
        },
    );

    Ok(result)
}
{% else %}
pub fn read_my_sb_message<TMessage: Send + Sync + Default + FromBytes>(
    msg: &MySbDeliveredMessage,
) -> Result<TMessage, DecodeError> {
    let result: TMessage = TMessage::from_bytes(msg.content.as_slice());

    let result = result;
    return Ok(result);
}

{% endif %}