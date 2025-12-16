use super::types::{ApiTestResult, TestReport};
use chrono::Utc;
use std::sync::{Arc, LazyLock, RwLock};
use std::time::Instant;

pub struct TestCollector {
    results: Arc<RwLock<Vec<ApiTestResult>>>,
    suite_name: RwLock<String>,
    start_time: RwLock<Option<Instant>>,
}

impl TestCollector {
    pub fn new() -> Self {
        Self {
            results: Arc::new(RwLock::new(Vec::new())),
            suite_name: RwLock::new("API Test Suite".to_string()),
            start_time: RwLock::new(None),
        }
    }

    pub fn global() -> &'static TestCollector {
        static GLOBAL: LazyLock<TestCollector> = LazyLock::new(TestCollector::new);
        &GLOBAL
    }

    pub fn set_suite_name(&self, name: impl Into<String>) {
        if let Ok(mut suite_name) = self.suite_name.write() {
            *suite_name = name.into();
        }
    }

    pub fn start(&self) {
        if let Ok(mut start_time) = self.start_time.write() {
            *start_time = Some(Instant::now());
        }
    }

    pub fn record(&self, result: ApiTestResult) {
        if let Ok(mut results) = self.results.write() {
            results.push(result);
        }
    }

    pub fn record_passed(
        &self,
        step_name: impl Into<String>,
        api_name: impl Into<String>,
        method: impl Into<String>,
        endpoint: impl Into<String>,
        status_code: u16,
        duration_ms: u64,
    ) {
        let result = ApiTestResult::passed(
            step_name,
            api_name,
            method,
            endpoint,
            status_code,
            duration_ms,
        );
        self.record(result);
    }

    pub fn record_failed(
        &self,
        step_name: impl Into<String>,
        api_name: impl Into<String>,
        method: impl Into<String>,
        endpoint: impl Into<String>,
        status_code: Option<u16>,
        duration_ms: u64,
        error: impl Into<String>,
    ) {
        let result = ApiTestResult::failed(
            step_name,
            api_name,
            method,
            endpoint,
            status_code,
            duration_ms,
            error,
        );
        self.record(result);
    }

    pub fn record_failed_with_details(
        &self,
        step_name: impl Into<String>,
        api_name: impl Into<String>,
        method: impl Into<String>,
        endpoint: impl Into<String>,
        status_code: Option<u16>,
        duration_ms: u64,
        error: impl Into<String>,
        request_body: Option<String>,
        response_body: Option<String>,
    ) {
        let mut result = ApiTestResult::failed(
            step_name,
            api_name,
            method,
            endpoint,
            status_code,
            duration_ms,
            error,
        );
        result.request_body = request_body;
        result.response_body = response_body;
        self.record(result);
    }

    pub fn generate_report(&self) -> TestReport {
        let suite_name = self
            .suite_name
            .read()
            .map(|s| s.clone())
            .unwrap_or_else(|_| "API Test Suite".to_string());

        let results = self
            .results
            .read()
            .map(|r| r.clone())
            .unwrap_or_else(|_| Vec::new());

        let mut report = TestReport::new(suite_name, results);

        if let Ok(start_time) = self.start_time.read() {
            if let Some(start) = *start_time {
                report.total_duration_ms = start.elapsed().as_millis() as u64;
            }
        }

        report.end_time = Utc::now();
        report
    }

    pub fn clear(&self) {
        if let Ok(mut results) = self.results.write() {
            results.clear();
        }
        if let Ok(mut start_time) = self.start_time.write() {
            *start_time = None;
        }
    }

    pub fn count(&self) -> usize {
        self.results.read().map(|r| r.len()).unwrap_or(0)
    }

    pub fn has_failures(&self) -> bool {
        self.results
            .read()
            .map(|r| r.iter().any(|result| result.status.is_failed()))
            .unwrap_or(false)
    }
}

impl Default for TestCollector {
    fn default() -> Self {
        Self::new()
    }
}
