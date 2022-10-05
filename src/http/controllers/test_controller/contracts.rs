use my_http_server_swagger::{MyHttpInput, MyHttpObjectStructure};
use serde::{Deserialize, Serialize};

#[derive(MyHttpInput)]
pub struct TestDataRequest {
    #[http_query(name = "Test"; description = "Test")]
    pub test: String,
}

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct TestDataResponse {
    pub result: String,
}