use serde::{Deserialize, Serialize};

use crate::tools::CodeReport;

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CodeReportJson {
    #[serde(default)]
    pub path: String,

    #[serde(default)]
    pub size: usize,

    #[serde(default)]
    pub has_allocator: bool,

    #[serde(default)]
    pub has_panic: String,
}

impl CodeReportJson {
    pub fn new(report: &CodeReport, size: usize) -> CodeReportJson {
        CodeReportJson {
            path: report.path.to_string_lossy().to_string(),
            size,
            has_allocator: report.has_allocator,
            has_panic: report.has_panic.to_string(),
        }
    }
}
