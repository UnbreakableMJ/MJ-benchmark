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

use std::error::Error;

/// Stub: later this will call Google Sheets API directly
pub fn append_row_to_sheet_stub(sheet_id: &str, row: &str) -> Result<(), Box<dyn Error>> {
    println!("[STUB] Would append to Google Sheet {}: {}", sheet_id, row);
    Ok(())
}

/// Stub: later this will call Google Drive API directly
pub fn upload_csv_to_drive_stub(folder_id: &str, csv_path: &str) -> Result<(), Box<dyn Error>> {
    println!(
        "[STUB] Would upload CSV {} to Google Drive folder {}",
        csv_path, folder_id
    );
    Ok(())
}