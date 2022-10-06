use std::sync::Arc;
{% if is_use_nosql == "writer" or is_use_nosql == "both" %}use my_no_sql_data_writer::MyNoSqlDataWriter;
use my_no_sql_server_abstractions::DataSyncronizationPeriod;{% endif %}
{% if is_use_nosql == "reader" or is_use_nosql == "both" %}use my_no_sql_tcp_reader::MyNoSqlDataReader;{% endif %}
{% if is_use_sb != "no" %}use my_service_bus_tcp_client::MyServiceBusClient;{% endif %}


{% if is_use_sb == "subscriber" or is_use_sb == "both" %}use my_service_bus_shared::queue::TopicQueueType;{% endif %}
use rust_extensions::AppStates;

use crate::{
    {% if is_use_psql %}TestDataRepository,{% endif %}
    SettingsModel,
    {% if is_use_nosql != "no" %}TestDataNoSqlEntity,
    TEST_DATA_NOSQL_TABLE_NAME,{% endif %}
    {% if is_use_grpc_client %}TestGrpcClient,{% endif %}
    {% if is_use_sb == "subscriber" or is_use_sb == "both" %}TEST_EVENT_SB_TOPIC_NAME,TestEventListener,{% endif %}
};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext {
    pub app_states: Arc<AppStates>,
    pub settings: SettingsModel,
    {% if is_use_psql %}pub orders_postgres_repo: TestDataRepository,{% endif %}
    {% if is_use_nosql == "writer" or is_use_nosql == "both" %}pub test_data_nosql_writer: MyNoSqlDataWriter<TestDataNoSqlEntity>,{% endif %}
    {% if is_use_nosql == "reader" or is_use_nosql == "both" %}pub test_data_nosql_reader: Arc<MyNoSqlDataReader<TestDataNoSqlEntity>>,{% endif %}
    {% if is_use_grpc_client %}pub test_grpc_service: TestGrpcClient,{% endif %}
    {% if is_use_sb != "no" %}pub my_sb_connection: MyServiceBusClient,{% endif %}

}

impl AppContext {
    pub async fn new(settings_reader: &Arc<crate::settings::SettingsReader>) -> Self {
        {% if is_use_psql %}let orders_postgres_repo = TestDataRepository::new(settings_reader.clone()).await;{% endif %}
        let settings = settings_reader.get_settings().await;
        let app_states = Arc::new(AppStates::create_initialized());

        {% if is_use_nosql == "writer" or is_use_nosql == "both" %}let test_data_nosql_writer = MyNoSqlDataWriter::new(
            settings.my_no_sql_data_writer.clone(),
            TEST_DATA_NOSQL_TABLE_NAME.to_owned(),
            true,
            true,
            DataSyncronizationPeriod::Sec5,
        );{% endif %}
        {% if is_use_nosql == "reader" or is_use_nosql == "both" %}
        let my_no_sql_connection = my_no_sql_tcp_reader::MyNoSqlTcpConnection::new(
            format!("{}:{}", APP_NAME, APP_VERSION),
            settings_reader.clone(),
        );

        let test_data_nosql_reader = my_no_sql_connection
            .get_reader(TEST_DATA_NOSQL_TABLE_NAME.to_owned())
            .await;{% endif %}

        {% if is_use_sb != "no" %}
        let my_sb_connection = MyServiceBusClient::new(
            APP_NAME,
            settings_reader.clone(),
            my_logger::LOGGER.clone(),
        );{% endif %}

        {% if is_use_grpc_client %}let test_grpc_service = TestGrpcClient::new(settings.grpc_service_url.clone()).await;{% endif %}

        Self {
            {% if is_use_psql %}orders_postgres_repo,{% endif %}
            app_states,
            settings,
            {% if is_use_nosql == "writer" or is_use_nosql == "both" %}test_data_nosql_writer,{% endif %}
            {% if is_use_nosql == "reader" or is_use_nosql == "both" %}test_data_nosql_reader,{% endif %}
            {% if is_use_grpc_client %}test_grpc_service,{% endif %}
            {% if is_use_sb != "no" %}my_sb_connection,{% endif %}

        }
    }
}

{% if is_use_sb == "subscriber" or is_use_sb == "both" %}
pub async fn bind_sb_subscribers(app: Arc<AppContext>) {
    app.my_sb_connection.subscribe(
        TEST_EVENT_SB_TOPIC_NAME.to_string(),
        APP_NAME.to_string(),
        TopicQueueType::DeleteOnDisconnect,
        Arc::new(TestEventListener::new(app.clone())),
    )
    .await;
}
{% endif %}