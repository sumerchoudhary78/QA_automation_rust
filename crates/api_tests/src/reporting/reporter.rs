use super::types::{TestReport, TestStatus};
use anyhow::Result;
use std::fs;
use std::path::Path;

pub trait Reporter {
    fn generate(&self, report: &TestReport) -> Result<String>;

    fn save(&self, report: &TestReport, path: &Path) -> Result<()> {
        let content = self.generate(report)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)?;
        Ok(())
    }
}

pub struct ConsoleReporter;

impl ConsoleReporter {
    pub fn new() -> Self {
        Self
    }

    pub fn print(&self, report: &TestReport) {
        println!();
        println!("╔══════════════════════════════════════════════════════════════════════╗");
        println!("║                         API TEST REPORT                              ║");
        println!("╠══════════════════════════════════════════════════════════════════════╣");
        println!(
            "║  Total: {:<4} │  Passed: {:<4} │  Failed: {:<4} │  Pass Rate: {:>5.1}%  ║",
            report.total_tests, report.passed, report.failed, report.pass_rate
        );
        println!("╚══════════════════════════════════════════════════════════════════════╝");
        println!();

        for result in &report.results {
            let status_icon = result.status.emoji();
            let status_code_str = result
                .status_code
                .map(|c| c.to_string())
                .unwrap_or_else(|| "---".to_string());
            let duration_str = format!("{}ms", result.duration_ms);

            println!(
                "{} {:<30} {:>4} {:<40} {:>6} {:>8}",
                status_icon,
                truncate(&result.step_name, 30),
                result.method,
                truncate(&result.endpoint, 40),
                status_code_str,
                duration_str
            );

            if result.status.is_failed() {
                if let Some(ref error) = result.error_message {
                    println!("   └─ Error: {}", truncate(error, 70));
                }
            }
        }

        println!();
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!(
            "📊 Suite: {} | Duration: {} | Executed: {}",
            report.suite_name,
            report.format_duration(),
            report.start_time.format("%Y-%m-%d %H:%M:%S UTC")
        );
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!();
    }
}

impl Default for ConsoleReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Reporter for ConsoleReporter {
    fn generate(&self, report: &TestReport) -> Result<String> {
        let mut output = String::new();

        output.push_str("\nAPI TEST REPORT\n");
        output.push_str(&format!(
            "Total: {} | Passed: {} | Failed: {} | Pass Rate: {:.1}%\n\n",
            report.total_tests, report.passed, report.failed, report.pass_rate
        ));

        for result in &report.results {
            let status = result.status.as_str();
            output.push_str(&format!(
                "[{}] {} - {} {} ({}ms)\n",
                status, result.step_name, result.method, result.endpoint, result.duration_ms
            ));
            if let Some(ref error) = result.error_message {
                output.push_str(&format!("    Error: {}\n", error));
            }
        }

        Ok(output)
    }
}

pub struct JsonReporter {
    pretty: bool,
}

impl JsonReporter {
    pub fn new() -> Self {
        Self { pretty: true }
    }

    pub fn compact() -> Self {
        Self { pretty: false }
    }
}

impl Default for JsonReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Reporter for JsonReporter {
    fn generate(&self, report: &TestReport) -> Result<String> {
        let json = if self.pretty {
            serde_json::to_string_pretty(report)?
        } else {
            serde_json::to_string(report)?
        };
        Ok(json)
    }
}

pub struct JUnitReporter;

impl JUnitReporter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for JUnitReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Reporter for JUnitReporter {
    fn generate(&self, report: &TestReport) -> Result<String> {
        let mut xml = String::new();
        xml.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        xml.push('\n');
        xml.push_str("<testsuites>\n");
        xml.push_str(&format!(
            r#"  <testsuite id="0" name="{}" package="api_tests" tests="{}" errors="0" failures="{}" hostname="localhost" timestamp="{}" time="{:.3}">"#,
            escape_xml(&report.suite_name),
            report.total_tests,
            report.failed,
            report.start_time.to_rfc3339(),
            report.total_duration_ms as f64 / 1000.0
        ));
        xml.push('\n');

        for result in &report.results {
            let time_secs = result.duration_ms as f64 / 1000.0;
            xml.push_str(&format!(
                r#"    <testcase name="{}" classname="{}" time="{:.3}">"#,
                escape_xml(&result.step_name),
                escape_xml(&result.api_name),
                time_secs
            ));
            xml.push('\n');

            if result.status == TestStatus::Failed {
                let error_msg = result.error_message.as_deref().unwrap_or("Unknown error");
                xml.push_str(&format!(
                    r#"      <failure type="API Error" message="{}">"#,
                    escape_xml(error_msg)
                ));

                xml.push_str("\nEndpoint: ");
                xml.push_str(&escape_xml(&format!(
                    "{} {}",
                    result.method, result.endpoint
                )));
                if let Some(ref req) = result.request_body {
                    xml.push_str("\nRequest: ");
                    xml.push_str(&escape_xml(req));
                }
                if let Some(ref resp) = result.response_body {
                    xml.push_str("\nResponse: ");
                    xml.push_str(&escape_xml(resp));
                }
                xml.push_str("</failure>\n");
            }

            xml.push_str("    </testcase>\n");
        }

        xml.push_str("  </testsuite>\n");
        xml.push_str("</testsuites>\n");

        Ok(xml)
    }
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
