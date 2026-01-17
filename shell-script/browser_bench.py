#!/usr/bin/env python3
# MJ Browser Benchmark Suite Logger
# Runs Speedometer, JetStream, MotionMark in headless Chromium and logs results into CSV

import csv
import os
from datetime import datetime
from playwright.sync_api import sync_playwright

CSV_FILE = os.path.expanduser("~/MJ_benchmarks.csv")

# Ensure CSV headers exist
if not os.path.exists(CSV_FILE):
    with open(CSV_FILE, "w", newline="") as f:
        writer = csv.writer(f)
        writer.writerow([
            "Date","Computer","CPU","GPU","NPU","RAM","Storage","Compilation Flags",
            "Distro","Shell","DE","Repo Level",
            "7-Zip MIPS","OpenSSL MB/s","RAMspeed MB/s",
            "fio Seq Read MB/s","fio Seq Write MB/s","fio Rand Read IOPS","fio Rand Write IOPS",
            "glmark2 Score","Kernel Build Time (s)",
            "Speedometer 2.1 Score","JetStream 2.2 Score","MotionMark 1.3 Score","Notes"
        ])

def run_benchmark(playwright, url, result_selector, browser="chromium"):
    browser = getattr(playwright, browser).launch(headless=True)
    page = browser.new_page()
    page.goto(url)
    # Click "Run" or "Start Test"
    try:
        page.click("#run-button")
    except:
        pass
    # Wait for result
    page.wait_for_selector(result_selector, timeout=600000)
    score = page.query_selector(result_selector).inner_text()
    browser.close()
    return score

with sync_playwright() as p:
    speedometer = run_benchmark(p, "https://browserbench.org/Speedometer2.1/", "#result-number")
    jetstream = run_benchmark(p, "https://browserbench.org/JetStream2.2/", ".benchmark-result")
    motionmark = run_benchmark(p, "https://browserbench.org/MotionMark1.3/", "#score")

# Metadata placeholders (replace with your logger integration)
date = datetime.now().strftime("%Y/%m/%d %H:%M")
computer = os.uname().nodename
cpu = "(fill from lscpu)"
gpu = "(fill from lspci)"
npu = "None"
ram = "(fill from free -h)"
storage = "(fill from lsblk)"
comp_flags = "(fill manually)"
distro = "(fill from /etc/os-release)"
shell = os.environ.get("SHELL","bash")
de = os.environ.get("XDG_CURRENT_DESKTOP","Unknown")
repo_level = "(fill manually)"

# Append row with browser scores only (other fields left blank for now)
with open(CSV_FILE, "a", newline="") as f:
    writer = csv.writer(f)
    writer.writerow([
        date,computer,cpu,gpu,npu,ram,storage,comp_flags,
        distro,shell,de,repo_level,
        "","","","","","","","",
        "",speedometer,jetstream,motionmark,"Browser run"
    ])

print("✅ Browser benchmark scores appended to", CSV_FILE)
