use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpOutput};
use crate::AppContext;

use super::contracts::*;

#[my_http_server_swagger::http_route(
    method: "GET",
    route: "/api/Test",
    description: "Test",
    controller: "TestController",
    input_data: "TestDataRequest",
    result:[
        {status_code: 200, description: "Ok response", model: "TestDataResponse"},
    ]
)]
pub struct TestControllerAction {
    app: Arc<AppContext>,
}

impl TestControllerAction {
    pub fn new(app: Arc<AppContext>) -> Self {
        Self { app }
    }
}

async fn handle_request(
    action: &TestControllerAction,
    input_data: TestDataRequest,
    ctx: &HttpContext,
) -> Result<HttpOkResult, HttpFailResult> {
    {% if is_use_telemetry %}let my_telemetry = ctx.telemetry_context;{% endif %}

    let response = TestDataResponse{
        result: input_data.test,
    };

    return HttpOutput::as_json(response).into_ok_result(true).into();
}
