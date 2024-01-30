use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SmartReportRequest {
    pub room: String,
    pub device: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmartReportResponce {
    pub room: String,
    pub device: String,
    pub report: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmartReportParams {
    pub request: Vec<SmartReportRequest>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SmartReport {
    pub reports: Vec<SmartReportResponce>,
}
