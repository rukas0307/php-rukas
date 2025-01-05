use ext_php_rs::prelude::*;
use reqwest::blocking::multipart;
use sha2::{Digest, Sha256};
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

// todo this is ugly as hell, make it more readable
#[php_function(ignore_module, name = "_0x0")]
pub fn _0x0(file: String, expires: Option<String>) -> String {
    const MIN_AGE: f64 = 30.0;
    const MAX_AGE: f64 = 365.0;
    const MAX_SIZE: f64 = 512.0 * 1024.0 * 1024.0;

    // Get the file metadata
    let metadata = match fs::metadata(&file) {
        Ok(meta) => meta,
        Err(_) => return String::from("Failed to read file metadata."),
    };

    let file_size = metadata.len() as f64;

    if file_size > MAX_SIZE {
        return String::from("File size exceeds the maximum allowed size of 512 MiB.");
    }

    // todo this part with expiration is not making sense, fix it with a bit simpler logic
    let retention_days =
        MIN_AGE + (MIN_AGE - MAX_AGE) * ((file_size / MAX_SIZE - 1.0).powi(3)).max(0.0);

    let retention_hours = (retention_days * 24.0).round() as i64;

    let file_part = match multipart::Part::file(&file) {
        Ok(part) => part,
        Err(_) => return String::from("Failed to read file for upload."),
    };

    let mut form = multipart::Form::new().part("file", file_part);

    if let Some(expiry) = expires {
        form = form.text("expires", expiry);
    } else {
        form = form.text("expires", retention_hours.to_string());
    }

    let mut hasher = Sha256::new();
    if let Ok(contents) = fs::read(&file) {
        hasher.update(contents);
    } else {
        return String::from("Failed to read file for hashing.");
    }
    let file_hash = format!("{:x}", hasher.finalize());
    let epoch_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let secret = format!("randomsecret_{}_{}", file_hash, epoch_time);
    form = form.text("secret", secret);

    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://0x0.st")
        .header("User-Agent", "PHP_Rukas/1.0")
        .multipart(form)
        .send();

    match response {
        Ok(res) => {
            if res.status().is_success() {
                match res.text() {
                    Ok(body) => body.trim().to_string(),
                    Err(_) => String::from("Failed to read response body."),
                }
            } else {
                format!("Failed to upload file. HTTP Status: {}", res.status())
            }
        }
        Err(_) => String::from("Failed to connect to the upload server."),
    }
}
