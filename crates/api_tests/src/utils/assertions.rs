use anyhow::{bail, Result};
use reqwest::{Response, StatusCode};
use serde_json::Value;

pub fn assert_success(response: &Response) -> Result<()> {
    let status = response.status();
    if !status.is_success() {
        bail!("Expected success status, got {}", status);
    }
    Ok(())
}

pub fn assert_status(response: &Response, expected: StatusCode) -> Result<()> {
    let actual = response.status();
    if actual != expected {
        bail!("Expected status {}, got {}", expected, actual);
    }
    Ok(())
}

pub fn assert_2xx(status: StatusCode) -> Result<()> {
    if !status.is_success() {
        bail!("Expected 2xx status code, got {}", status);
    }
    Ok(())
}

pub fn assert_ok(status: StatusCode) -> Result<()> {
    if status != StatusCode::OK {
        bail!("Expected 200 OK, got {}", status);
    }
    Ok(())
}

pub fn assert_created(status: StatusCode) -> Result<()> {
    if status != StatusCode::CREATED {
        bail!("Expected 201 Created, got {}", status);
    }
    Ok(())
}

pub fn assert_bad_request(status: StatusCode) -> Result<()> {
    if status != StatusCode::BAD_REQUEST {
        bail!("Expected 400 Bad Request, got {}", status);
    }
    Ok(())
}

pub fn assert_unauthorized(status: StatusCode) -> Result<()> {
    if status != StatusCode::UNAUTHORIZED {
        bail!("Expected 401 Unauthorized, got {}", status);
    }
    Ok(())
}

pub fn assert_not_found(status: StatusCode) -> Result<()> {
    if status != StatusCode::NOT_FOUND {
        bail!("Expected 404 Not Found, got {}", status);
    }
    Ok(())
}

pub fn assert_header(response: &Response, header_name: &str) -> Result<()> {
    if response.headers().get(header_name).is_none() {
        bail!("Expected header '{}' not found", header_name);
    }
    Ok(())
}

pub fn assert_header_value(
    response: &Response,
    header_name: &str,
    expected_value: &str,
) -> Result<()> {
    let actual_value = response
        .headers()
        .get(header_name)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if actual_value != expected_value {
        bail!(
            "Header '{}' expected value '{}', got '{}'",
            header_name,
            expected_value,
            actual_value
        );
    }
    Ok(())
}

pub fn assert_json_field(json: &Value, field_path: &str) -> Result<()> {
    let parts: Vec<&str> = field_path.split('.').collect();
    let mut current = json;

    for part in parts {
        current = current.get(part).ok_or_else(|| {
            anyhow::anyhow!("JSON field '{}' not found in path '{}'", part, field_path)
        })?;
    }

    Ok(())
}

pub fn assert_json_value(json: &Value, field_path: &str, expected: &Value) -> Result<()> {
    let parts: Vec<&str> = field_path.split('.').collect();
    let mut current = json;

    for part in parts {
        current = current.get(part).ok_or_else(|| {
            anyhow::anyhow!("JSON field '{}' not found in path '{}'", part, field_path)
        })?;
    }

    if current != expected {
        bail!(
            "JSON field '{}' expected value {:?}, got {:?}",
            field_path,
            expected,
            current
        );
    }

    Ok(())
}

pub fn assert_response_time(elapsed_ms: u128, max_ms: u128) -> Result<()> {
    if elapsed_ms > max_ms {
        bail!(
            "Response time {}ms exceeded threshold {}ms",
            elapsed_ms,
            max_ms
        );
    }
    Ok(())
}
