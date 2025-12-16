use super::reporter::Reporter;
use super::types::{TestReport, TestStatus};
use anyhow::Result;

pub struct HtmlReporter;

impl HtmlReporter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for HtmlReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Reporter for HtmlReporter {
    fn generate(&self, report: &TestReport) -> Result<String> {
        let pass_color = if report.pass_rate >= 90.0 {
            "#10b981"
        } else if report.pass_rate >= 70.0 {
            "#f59e0b"
        } else {
            "#ef4444"
        };

        let html = format!(
            r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>API Test Report - {suite_name}</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
            background: linear-gradient(135deg, #0f0f23 0%, #1a1a3e 50%, #0f0f23 100%);
            min-height: 100vh;
            color: #e0e0e0;
            padding: 2rem;
        }}
        
        .container {{
            max-width: 1400px;
            margin: 0 auto;
        }}
        
        header {{
            text-align: center;
            margin-bottom: 2rem;
        }}
        
        h1 {{
            font-size: 2.5rem;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
            margin-bottom: 0.5rem;
        }}
        
        .timestamp {{
            color: #888;
            font-size: 0.9rem;
        }}
        
        .summary-cards {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 1.5rem;
            margin-bottom: 2rem;
        }}
        
        .card {{
            background: rgba(255, 255, 255, 0.05);
            backdrop-filter: blur(10px);
            border: 1px solid rgba(255, 255, 255, 0.1);
            border-radius: 16px;
            padding: 1.5rem;
            text-align: center;
            transition: transform 0.3s ease, box-shadow 0.3s ease;
        }}
        
        .card:hover {{
            transform: translateY(-4px);
            box-shadow: 0 8px 32px rgba(102, 126, 234, 0.3);
        }}
        
        .card-value {{
            font-size: 2.5rem;
            font-weight: 700;
            margin-bottom: 0.5rem;
        }}
        
        .card-label {{
            font-size: 0.875rem;
            text-transform: uppercase;
            letter-spacing: 1px;
            color: #888;
        }}
        
        .card.total .card-value {{ color: #667eea; }}
        .card.passed .card-value {{ color: #10b981; }}
        .card.failed .card-value {{ color: #ef4444; }}
        .card.rate .card-value {{ color: {pass_color}; }}
        
        .filters {{
            display: flex;
            gap: 1rem;
            margin-bottom: 1.5rem;
            flex-wrap: wrap;
        }}
        
        .filter-btn {{
            padding: 0.75rem 1.5rem;
            border: none;
            border-radius: 8px;
            background: rgba(255, 255, 255, 0.05);
            color: #e0e0e0;
            cursor: pointer;
            transition: all 0.3s ease;
            font-size: 0.9rem;
        }}
        
        .filter-btn:hover {{
            background: rgba(255, 255, 255, 0.1);
        }}
        
        .filter-btn.active {{
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
        }}
        
        .results-table {{
            background: rgba(255, 255, 255, 0.02);
            border-radius: 16px;
            overflow: hidden;
            border: 1px solid rgba(255, 255, 255, 0.1);
        }}
        
        table {{
            width: 100%;
            border-collapse: collapse;
        }}
        
        th {{
            background: rgba(102, 126, 234, 0.2);
            padding: 1rem;
            text-align: left;
            font-weight: 600;
            text-transform: uppercase;
            font-size: 0.75rem;
            letter-spacing: 1px;
        }}
        
        td {{
            padding: 1rem;
            border-bottom: 1px solid rgba(255, 255, 255, 0.05);
        }}
        
        tr:hover {{
            background: rgba(255, 255, 255, 0.02);
        }}
        
        tr.passed td:first-child {{
            border-left: 4px solid #10b981;
        }}
        
        tr.failed td:first-child {{
            border-left: 4px solid #ef4444;
        }}
        
        .status-badge {{
            display: inline-flex;
            align-items: center;
            gap: 0.5rem;
            padding: 0.375rem 0.75rem;
            border-radius: 9999px;
            font-size: 0.75rem;
            font-weight: 600;
        }}
        
        .status-badge.passed {{
            background: rgba(16, 185, 129, 0.2);
            color: #10b981;
        }}
        
        .status-badge.failed {{
            background: rgba(239, 68, 68, 0.2);
            color: #ef4444;
        }}
        
        .method-badge {{
            padding: 0.25rem 0.5rem;
            border-radius: 4px;
            font-size: 0.75rem;
            font-weight: 700;
            font-family: monospace;
        }}
        
        .method-GET {{ background: #10b981; color: white; }}
        .method-POST {{ background: #3b82f6; color: white; }}
        .method-PUT {{ background: #f59e0b; color: white; }}
        .method-PATCH {{ background: #8b5cf6; color: white; }}
        .method-DELETE {{ background: #ef4444; color: white; }}
        
        .endpoint {{
            font-family: monospace;
            font-size: 0.85rem;
            color: #a5b4fc;
            word-break: break-all;
        }}
        
        .duration {{
            color: #888;
            font-family: monospace;
        }}
        
        .error-details {{
            display: none;
            padding: 1rem;
            background: rgba(239, 68, 68, 0.1);
            border-radius: 8px;
            margin-top: 0.5rem;
            font-family: monospace;
            font-size: 0.85rem;
            white-space: pre-wrap;
            word-break: break-all;
        }}
        
        .error-details.show {{
            display: block;
        }}
        
        .toggle-details {{
            cursor: pointer;
            color: #667eea;
            font-size: 0.85rem;
        }}
        
        .toggle-details:hover {{
            text-decoration: underline;
        }}
        
        .request-response {{
            margin-top: 1rem;
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 1rem;
        }}
        
        .request-response > div {{
            background: rgba(0, 0, 0, 0.3);
            border-radius: 8px;
            padding: 1rem;
            overflow: auto;
            max-height: 300px;
        }}
        
        .request-response h4 {{
            color: #888;
            margin-bottom: 0.5rem;
            font-size: 0.75rem;
            text-transform: uppercase;
        }}
        
        .request-response pre {{
            color: #e0e0e0;
            font-size: 0.8rem;
            white-space: pre-wrap;
        }}

        footer {{
            margin-top: 2rem;
            text-align: center;
            color: #666;
            font-size: 0.85rem;
        }}
        
        @media (max-width: 768px) {{
            .request-response {{
                grid-template-columns: 1fr;
            }}
            
            table {{
                font-size: 0.85rem;
            }}
            
            th, td {{
                padding: 0.75rem 0.5rem;
            }}
        }}
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>🔬 {suite_name}</h1>
            <p class="timestamp">Executed on {execution_time} • Duration: {duration}</p>
        </header>
        
        <section class="summary-cards">
            <div class="card total">
                <div class="card-value">{total}</div>
                <div class="card-label">Total Tests</div>
            </div>
            <div class="card passed">
                <div class="card-value">{passed}</div>
                <div class="card-label">Passed</div>
            </div>
            <div class="card failed">
                <div class="card-value">{failed}</div>
                <div class="card-label">Failed</div>
            </div>
            <div class="card rate">
                <div class="card-value">{pass_rate:.1}%</div>
                <div class="card-label">Pass Rate</div>
            </div>
        </section>
        
        <section class="filters">
            <button class="filter-btn active" onclick="filterResults('all')">All ({total})</button>
            <button class="filter-btn" onclick="filterResults('passed')">✅ Passed ({passed})</button>
            <button class="filter-btn" onclick="filterResults('failed')">❌ Failed ({failed})</button>
        </section>
        
        <section class="results-table">
            <table>
                <thead>
                    <tr>
                        <th>Step</th>
                        <th>Method</th>
                        <th>Endpoint</th>
                        <th>Status Code</th>
                        <th>Duration</th>
                        <th>Result</th>
                    </tr>
                </thead>
                <tbody>
                    {rows}
                </tbody>
            </table>
        </section>
        
        <footer>
            <p>Generated by API Test Reporter • {execution_time}</p>
        </footer>
    </div>
    
    <script>
        function filterResults(filter) {{
            const rows = document.querySelectorAll('tbody tr');
            const buttons = document.querySelectorAll('.filter-btn');
            
            buttons.forEach(btn => btn.classList.remove('active'));
            event.target.classList.add('active');
            
            rows.forEach(row => {{
                if (filter === 'all') {{
                    row.style.display = '';
                }} else if (row.classList.contains(filter)) {{
                    row.style.display = '';
                }} else {{
                    row.style.display = 'none';
                }}
            }});
        }}
        
        function toggleDetails(id) {{
            const details = document.getElementById(id);
            details.classList.toggle('show');
        }}
    </script>
</body>
</html>"##,
            suite_name = report.suite_name,
            execution_time = report.start_time.format("%Y-%m-%d %H:%M:%S UTC"),
            duration = report.format_duration(),
            total = report.total_tests,
            passed = report.passed,
            failed = report.failed,
            pass_rate = report.pass_rate,
            pass_color = pass_color,
            rows = self.generate_rows(report),
        );

        Ok(html)
    }
}

impl HtmlReporter {
    fn generate_rows(&self, report: &TestReport) -> String {
        let mut rows = String::new();

        for (idx, result) in report.results.iter().enumerate() {
            let status_class = match result.status {
                TestStatus::Passed => "passed",
                TestStatus::Failed => "failed",
                TestStatus::Skipped => "skipped",
            };

            let status_badge = match result.status {
                TestStatus::Passed => r#"<span class="status-badge passed">✅ PASSED</span>"#,
                TestStatus::Failed => r#"<span class="status-badge failed">❌ FAILED</span>"#,
                TestStatus::Skipped => r#"<span class="status-badge">⏭️ SKIPPED</span>"#,
            };

            let status_code_display = result
                .status_code
                .map(|c| c.to_string())
                .unwrap_or_else(|| "—".to_string());

            let error_section = if result.status.is_failed() {
                let error_msg = result.error_message.as_deref().unwrap_or("Unknown error");

                let mut detail_html = format!(
                    r#"<div class="error-details" id="error-{idx}">
<strong>Error:</strong> {error}
"#,
                    idx = idx,
                    error = escape_html(error_msg)
                );

                if result.request_body.is_some() || result.response_body.is_some() {
                    detail_html.push_str(r#"<div class="request-response">"#);

                    if let Some(ref req) = result.request_body {
                        detail_html.push_str(&format!(
                            r#"<div><h4>Request Body</h4><pre>{}</pre></div>"#,
                            escape_html(req)
                        ));
                    }

                    if let Some(ref resp) = result.response_body {
                        detail_html.push_str(&format!(
                            r#"<div><h4>Response Body</h4><pre>{}</pre></div>"#,
                            escape_html(resp)
                        ));
                    }

                    detail_html.push_str("</div>");
                }

                detail_html.push_str("</div>");

                format!(
                    r#"<br><span class="toggle-details" onclick="toggleDetails('error-{idx}')">▶ View Details</span>{detail_html}"#,
                    idx = idx,
                    detail_html = detail_html
                )
            } else {
                String::new()
            };

            rows.push_str(&format!(
                r#"<tr class="{status_class}">
    <td>{step_name}{error_section}</td>
    <td><span class="method-badge method-{method}">{method}</span></td>
    <td class="endpoint">{endpoint}</td>
    <td>{status_code}</td>
    <td class="duration">{duration}ms</td>
    <td>{status_badge}</td>
</tr>
"#,
                status_class = status_class,
                step_name = escape_html(&result.step_name),
                error_section = error_section,
                method = result.method,
                endpoint = escape_html(&result.endpoint),
                status_code = status_code_display,
                duration = result.duration_ms,
                status_badge = status_badge,
            ));
        }

        rows
    }
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
