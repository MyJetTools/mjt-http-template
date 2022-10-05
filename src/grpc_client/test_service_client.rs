use std::{time::Duration};

{% if is_use_telemetry%}use my_grpc_extensions::GrpcClientInterceptor;{% endif %}
{% if is_seq_enabled %}use my_telemetry::MyTelemetryContext;{% endif %}
use tonic::{{% if is_use_telemetry%}codegen::InterceptedService,{% endif %} transport::Channel};

use crate::testgrpc_grpc::{test_service_client::TestServiceClient, TestResponse, TestRequest};

pub struct TestGrpcClient {
    channel: Channel,
    timeout: Duration,
}

impl TestGrpcClient {
    pub async fn new(grpc_address: String) -> Self {
        let channel = Channel::from_shared(grpc_address)
            .unwrap()
            .connect()
            .await
            .unwrap();
        Self {
            channel,
            timeout: Duration::from_secs(5),
        }
    }

    {% if is_use_telemetry%}fn create_grpc_service(
        &self,
        {% if is_seq_enabled %}my_telemetry_context: &MyTelemetryContext,{% endif %}
    ) -> TestServiceClient<
        InterceptedService<Channel, GrpcClientInterceptor>,
    > {
        return TestServiceClient::with_interceptor(
            self.channel.clone(),
            GrpcClientInterceptor::new(my_telemetry_context),
        );
    }{% else %}
    fn create_grpc_service(
        &self
    ) -> TestServiceClient<Channel>
     {
        return TestServiceClient::new(
            self.channel.clone()
        );
    }
    {% endif %}

    pub async fn test(
        &self,
        test: String,
        {% if is_use_telemetry%}my_telemetry_context: &MyTelemetryContext,{% endif %}
    ) -> Option<TestResponse> {
        let request = TestRequest {
            test
        };

        let mut client = self.create_grpc_service({% if is_use_telemetry%}my_telemetry_context{% endif %});
        let future = client.test(request);
        let response = tokio::time::timeout(self.timeout, future)
            .await
            .unwrap()
            .unwrap();

        let response = response.into_inner();

        Some(TestResponse{
            test: response.test
        })
    }
}
