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
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SheetsError {
    #[error("HTTP error: {0}")]
    Http(String),
}

pub async fn append_row(
    sheet_id: &str,
    row: &str,
    token: &StoredToken,
) -> Result<(), SheetsError> {
    let client = Client::new();

    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/A1:append?valueInputOption=USER_ENTERED",
        sheet_id
    );

    let body = json!({
        "values": [
            row.split(',').collect::<Vec<_>>()
        ]
    });

    let res = client
        .post(url)
        .bearer_auth(&token.access_token)
        .json(&body)
        .send()
        .await
        .map_err(|e| SheetsError::Http(e.to_string()))?;

    if !res.status().is_success() {
        return Err(SheetsError::Http(format!(
            "Sheets API error: {}",
            res.text().await.unwrap_or_default()
        )));
    }

    Ok(())
}