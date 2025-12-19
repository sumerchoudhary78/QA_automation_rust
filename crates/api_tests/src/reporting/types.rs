use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TestStatus {
    Passed,
    Failed,
    Skipped,
}

impl TestStatus {
    pub fn is_passed(&self) -> bool {
        matches!(self, TestStatus::Passed)
    }

    pub fn is_failed(&self) -> bool {
        matches!(self, TestStatus::Failed)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            TestStatus::Passed => "PASSED",
            TestStatus::Failed => "FAILED",
            TestStatus::Skipped => "SKIPPED",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            TestStatus::Passed => "✅",
            TestStatus::Failed => "❌",
            TestStatus::Skipped => "⏭️",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiTestResult {
    pub step_name: String,

    pub api_name: String,

    pub method: String,

    pub endpoint: String,

    pub status_code: Option<u16>,

    pub duration_ms: u64,

    pub status: TestStatus,

    pub error_message: Option<String>,

    pub request_body: Option<String>,

    pub response_body: Option<String>,

    pub timestamp: DateTime<Utc>,
}

impl ApiTestResult {
    pub fn passed(
        step_name: impl Into<String>,
        api_name: impl Into<String>,
        method: impl Into<String>,
        endpoint: impl Into<String>,
        status_code: u16,
        duration_ms: u64,
    ) -> Self {
        Self {
            step_name: step_name.into(),
            api_name: api_name.into(),
            method: method.into(),
            endpoint: endpoint.into(),
            status_code: Some(status_code),
            duration_ms,
            status: TestStatus::Passed,
            error_message: None,
            request_body: None,
            response_body: None,
            timestamp: Utc::now(),
        }
    }

    pub fn failed(
        step_name: impl Into<String>,
        api_name: impl Into<String>,
        method: impl Into<String>,
        endpoint: impl Into<String>,
        status_code: Option<u16>,
        duration_ms: u64,
        error: impl Into<String>,
    ) -> Self {
        Self {
            step_name: step_name.into(),
            api_name: api_name.into(),
            method: method.into(),
            endpoint: endpoint.into(),
            status_code,
            duration_ms,
            status: TestStatus::Failed,
            error_message: Some(error.into()),
            request_body: None,
            response_body: None,
            timestamp: Utc::now(),
        }
    }

    pub fn with_request_body(mut self, body: impl Into<String>) -> Self {
        self.request_body = Some(body.into());
        self
    }

    pub fn with_response_body(mut self, body: impl Into<String>) -> Self {
        self.response_body = Some(body.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestReport {
    pub suite_name: String,

    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,

    pub total_duration_ms: u64,

    pub results: Vec<ApiTestResult>,

    pub total_tests: usize,

    pub passed: usize,

    pub failed: usize,
    pub skipped: usize,
    pub pass_rate: f64,
}

impl TestReport {
    pub fn new(suite_name: impl Into<String>, results: Vec<ApiTestResult>) -> Self {
        let total_tests = results.len();
        let passed = results.iter().filter(|r| r.status.is_passed()).count();
        let failed = results.iter().filter(|r| r.status.is_failed()).count();
        let skipped = total_tests - passed - failed;
        let pass_rate = if total_tests > 0 {
            (passed as f64 / total_tests as f64) * 100.0
        } else {
            0.0
        };

        let total_duration_ms: u64 = results.iter().map(|r| r.duration_ms).sum();

        let start_time = results
            .first()
            .map(|r| r.timestamp)
            .unwrap_or_else(Utc::now);
        let end_time = results.last().map(|r| r.timestamp).unwrap_or_else(Utc::now);

        Self {
            suite_name: suite_name.into(),
            start_time,
            end_time,
            total_duration_ms,
            results,
            total_tests,
            passed,
            failed,
            skipped,
            pass_rate,
        }
    }

    pub fn failed_tests(&self) -> Vec<&ApiTestResult> {
        self.results
            .iter()
            .filter(|r| r.status.is_failed())
            .collect()
    }

    pub fn passed_tests(&self) -> Vec<&ApiTestResult> {
        self.results
            .iter()
            .filter(|r| r.status.is_passed())
            .collect()
    }

    pub fn has_failures(&self) -> bool {
        self.failed > 0
    }

    pub fn format_duration(&self) -> String {
        let secs = self.total_duration_ms / 1000;
        let ms = self.total_duration_ms % 1000;
        if secs > 0 {
            format!("{}.{:03}s", secs, ms)
        } else {
            format!("{}ms", ms)
        }
    }
}
