use std::sync::Arc;
{% if is_use_nosql == "writer" or is_use_nosql == "both" %}use my_no_sql_data_writer::MyNoSqlDataWriter;
use my_no_sql_server_abstractions::DataSyncronizationPeriod;{% endif %}
{% if is_use_nosql == "reader" or is_use_nosql == "both" %}use my_no_sql_tcp_reader::MyNoSqlDataReader;{% endif %}
use rust_extensions::AppStates;

use crate::{
    {% if is_use_psql %}TestDataRepository,{% endif %}
    SettingsModel,
    {% if is_use_nosql != "no" %}TestDataNoSqlEntity,
    TEST_DATA_NOSQL_TABLE_NAME,{% endif %}
    {% if is_use_grpc_client %}TestGrpcClient{% endif %}
};

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

pub struct AppContext {
    pub app_states: Arc<AppStates>,
    pub settings: SettingsModel,
    {% if is_use_psql %}pub orders_postgres_repo: TestDataRepository,{% endif %}
    {% if is_use_nosql == "writer" or is_use_nosql == "both" %}pub test_data_nosql_writer: MyNoSqlDataWriter<TestDataNoSqlEntity>,{% endif %}
    {% if is_use_nosql == "reader" or is_use_nosql == "both" %}pub test_data_nosql_reader: MyNoSqlDataReader<TestDataNoSqlEntity>,{% endif %}
    {% if is_use_grpc_client %}pub test_grpc_service: TestGrpcClient,{% endif %}
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
        let test_data_nosql_reader = MyNoSqlDataReader::new(
            TEST_DATA_NOSQL_TABLE_NAME.to_owned(),
            app_states.clone(),
        ).await;{% endif %}

        {% if is_use_grpc_client %}let test_grpc_service = TestGrpcClient::new(settings.grpc_service_url.clone()).await;{% endif %}

        Self {
            {% if is_use_psql %}orders_postgres_repo,{% endif %}
            app_states,
            settings,
            {% if is_use_nosql == "writer" or is_use_nosql == "both" %}test_data_nosql_writer,{% endif %}
            {% if is_use_nosql == "reader" or is_use_nosql == "both" %}test_data_nosql_reader,{% endif %}
            {% if is_use_grpc_client %}test_grpc_service,{% endif %}
        }
    }
}
