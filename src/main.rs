use std::sync::Arc;

use {{project-name | snake_case}}::{SettingsReader, APP_NAME, APP_VERSION, AppContext, setup_server};

{% if is_seq_enabled %}use my_seq_logger::SeqLogger;{% endif %}

#[tokio::main]
async fn main() {
    let settings_reader = SettingsReader::new(".{{project-name}}").await;
    let settings_reader = Arc::new(settings_reader);

    {% if is_seq_enabled %}SeqLogger::enable_from_connection_string(
        APP_NAME.to_string(),
        APP_VERSION.to_string(),
        settings_reader.clone(),
        None,
    );{% endif %}

    let app = AppContext::new(&settings_reader).await;
    let app = Arc::new(app);

    {% if is_use_telemetry %}let telemetry_writer = my_telemetry_writer::MyTelemetryWriter::new(
        APP_NAME.to_string(),
        settings_reader.clone(),
    );
    telemetry_writer.start(app.app_states.clone(), my_logger::LOGGER.clone());{% endif %}
    
    setup_server(app.clone(), {{http_port}});
    app.app_states.wait_until_shutdown().await;
}
