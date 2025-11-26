use anyhow::{Context, Result};
use serde_json::Value;
use std::fs;
use thirtyfour::prelude::*;

pub async fn save_session(driver: &WebDriver, prefix: &str) -> Result<()> {
    let local_storage: Value = driver
        .execute(
            r#"
        const out = {};
        for (let i = 0; i < localStorage.length; i++) {
            const k = localStorage.key(i);
            out[k] = localStorage.getItem(k);
        }
        return out;
        "#,
            Vec::<serde_json::Value>::new(),
        )
        .await?
        .convert()?;

    let ls_file = format!("{}_local_storage.json", prefix);
    fs::write(&ls_file, serde_json::to_string_pretty(&local_storage)?)?;

    let cookies = driver.get_all_cookies().await?;
    let ck_file = format!("{}_cookies.json", prefix);
    let ck_json = serde_json::to_string_pretty(&cookies)?;
    fs::write(&ck_file, ck_json)?;

    Ok(())
}

pub async fn restore_session(driver: &WebDriver, prefix: &str, base_url: &str) -> Result<()> {
    driver
        .goto(base_url)
        .await
        .context("goto base_url failed")?;
    let ck_file = format!("{}_cookies.json", prefix);
    if let Ok(ck_contents) = fs::read_to_string(&ck_file) {
        let cookies: Vec<thirtyfour::Cookie> =
            serde_json::from_str(&ck_contents).context("failed to parse cookies file")?;
        for cookie in cookies {
            driver.add_cookie(cookie).await?;
        }
    }

    let ls_file = format!("{}_local_storage.json", prefix);
    if let Ok(ls_contents) = fs::read_to_string(&ls_file) {
        let map: serde_json::Map<String, Value> =
            serde_json::from_str(&ls_contents).context("failed to parse local storage file")?;
        for (k, v) in map {
            let value_str = v
                .as_str()
                .map(|s| s.to_string())
                .unwrap_or_else(|| v.to_string());
            driver
                .execute(
                    "window.localStorage.setItem(arguments[0], arguments[1]);",
                    vec![
                        serde_json::Value::String(k),
                        serde_json::Value::String(value_str),
                    ],
                )
                .await?;
        }
    }
    driver.refresh().await?;
    Ok(())
}

pub async fn check_logged_in(driver: &WebDriver, check_url: &str) -> Result<bool> {
    let script = r#"
        return fetch(arguments[0], { credentials: 'include' })
            .then(r => ({ ok: r.ok, status: r.status }))
            .catch(e => ({ ok: false, status: 0 }));
    "#;

    let res: serde_json::Value = driver
        .execute(
            script,
            vec![serde_json::Value::String(check_url.to_string())],
        )
        .await?
        .convert()?;
    let ok = res.get("ok").and_then(|v| v.as_bool()).unwrap_or(false);
    Ok(ok)
}
