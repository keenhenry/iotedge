/*
 * IoT Edge Management API
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 2018-06-28
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExitStatus {
    #[serde(rename = "exitTime")]
    exit_time: String,
    #[serde(rename = "statusCode")]
    status_code: String,
}

impl ExitStatus {
    pub fn new(exit_time: String, status_code: String) -> ExitStatus {
        ExitStatus {
            exit_time,
            status_code,
        }
    }

    pub fn set_exit_time(&mut self, exit_time: String) {
        self.exit_time = exit_time;
    }

    pub fn with_exit_time(mut self, exit_time: String) -> ExitStatus {
        self.exit_time = exit_time;
        self
    }

    pub fn exit_time(&self) -> &String {
        &self.exit_time
    }

    pub fn set_status_code(&mut self, status_code: String) {
        self.status_code = status_code;
    }

    pub fn with_status_code(mut self, status_code: String) -> ExitStatus {
        self.status_code = status_code;
        self
    }

    pub fn status_code(&self) -> &String {
        &self.status_code
    }
}
