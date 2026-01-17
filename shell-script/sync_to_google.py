#!/usr/bin/env python3
# Sync MJ benchmark CSV to Google Drive + Google Sheets

import os
import argparse
from google.oauth2.credentials import Credentials
from google_auth_oauthlib.flow import InstalledAppFlow
from googleapiclient.discovery import build
from googleapiclient.http import MediaFileUpload

SCOPES = [
    "https://www.googleapis.com/auth/drive.file",
    "https://www.googleapis.com/auth/spreadsheets"
]

parser = argparse.ArgumentParser()
parser.add_argument("--csv", required=True, help="Path to master CSV")
parser.add_argument("--sheet", required=True, help="Google Sheet ID")
parser.add_argument("--drive-folder", required=True, help="Google Drive folder ID")
args = parser.parse_args()

CSV_FILE = args.csv
SHEET_ID = args.sheet
FOLDER_ID = args.drive_folder

# --- Auth ---
creds = None
token_path = os.path.expanduser("~/.mohamed_google_token.json")
cred_path = os.path.expanduser("~/mohamed_bench_sync/credentials.json")

if os.path.exists(token_path):
    creds = Credentials.from_authorized_user_file(token_path, SCOPES)
else:
    flow = InstalledAppFlow.from_client_secrets_file(cred_path, SCOPES)
    creds = flow.run_local_server(port=0)
    with open(token_path, "w") as token:
        token.write(creds.to_json())

drive = build("drive", "v3", credentials=creds)
sheets = build("sheets", "v4", credentials=creds)

# --- Upload CSV to Google Drive ---
file_metadata = {
    "name": "mj_benchmarks.csv",
    "parents": [FOLDER_ID]
}

media = MediaFileUpload(CSV_FILE, mimetype="text/csv", resumable=True)

drive.files().create(
    body=file_metadata,
    media_body=media,
    fields="id"
).execute()

print("Uploaded CSV to Google Drive.")

# --- Append last row to Google Sheet ---
with open(CSV_FILE) as f:
    last_line = f.readlines()[-1].strip()

values = [last_line.split(",")]

sheets.spreadsheets().values().append(
    spreadsheetId=SHEET_ID,
    range="A1",
    valueInputOption="RAW",
    insertDataOption="INSERT_ROWS",
    body={"values": values}
).execute()

print("Appended row to Google Sheet.")