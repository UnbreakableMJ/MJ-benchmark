// SPDX-License-Identifier: GPL-3.0-or-later
//
// MJ Benchmark
// Copyright (c) 2024-2026
// Mohamed Hammad
//
// Trademarks:
//   Steelbore, S3cure, S3cure me, S3cure us, MJ Benchmark
//   These names are trademarks of Mohamed Hammad and may not be used
//   to endorse or promote derivative products without prior permission.

use crate::google_auth::StoredToken;
use reqwest::Client;
use serde_json::json;
use std::fs;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DriveError {
    #[error("HTTP error: {0}")]
    Http(String),
}

pub async fn upload_csv(
    folder_id: &str,
    csv_path: &str,
    token: &StoredToken,
) -> Result<(), DriveError> {
    let client = Client::new();
    let file_bytes = fs::read(csv_path).map_err(|e| DriveError::Http(e.to_string()))?;

    let metadata = json!({
        "name": "mj_benchmarks.csv",
        "parents": [folder_id]
    });

    let url = "https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart";

    let form = reqwest::multipart::Form::new()
        .part(
            "metadata",
            reqwest::multipart::Part::text(metadata.to_string())
                .mime_str("application/json")
                .unwrap(),
        )
        .part(
            "file",
            reqwest::multipart::Part::bytes(file_bytes)
                .mime_str("text/csv")
                .unwrap(),
        );

    let res = client
        .post(url)
        .bearer_auth(&token.access_token)
        .multipart(form)
        .send()
        .await
        .map_err(|e| DriveError::Http(e.to_string()))?;

    if !res.status().is_success() {
        return Err(DriveError::Http(format!(
            "Drive API error: {}",
            res.text().await.unwrap_or_default()
        )));
    }

    Ok(())
}