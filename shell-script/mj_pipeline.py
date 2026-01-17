#!/usr/bin/env python3
# MJ Unified Benchmark Pipeline
# Runs PTS suite + browser benchmarks + metadata logging into CSV

import csv, os, subprocess, json
from datetime import datetime
from playwright.sync_api import sync_playwright

CSV_FILE = os.path.expanduser("~/MJ_benchmarks.csv")
RUN_NAME = "MJ-core"

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

# --- Run PTS suite ---
subprocess.run(["phoronix-test-suite", "benchmark", RUN_NAME])

# Parse PTS JSON results
result_dir = os.path.expanduser(f"~/.phoronix-test-suite/test-results/{RUN_NAME}")
result_json = os.path.join(result_dir, "results.json")
with open(result_json) as f:
    results = json.load(f)

def get_result(identifier):
    for r in results["Results"]:
        if r["Identifier"] == identifier:
            return r["Result"]
    return ""

sevenzip = get_result("pts/compress-7zip")
openssl = get_result("pts/openssl")
ramspeed = get_result("pts/ramspeed")
fio = get_result("pts/fio")
glmark2 = get_result("pts/glmark2")
kernel_build = get_result("pts/build-linux-kernel")

# --- Run browser benchmarks ---
def run_browser_bench(playwright, url, selector):
    browser = playwright.chromium.launch(headless=True)
    page = browser.new_page()
    page.goto(url)
    try:
        page.click("#run-button")
    except:
        pass
    page.wait_for_selector(selector, timeout=600000)
    score = page.query_selector(selector).inner_text()
    browser.close()
    return score

with sync_playwright() as p:
    speedometer = run_browser_bench(p, "https://browserbench.org/Speedometer2.1/", "#result-number")
    jetstream = run_browser_bench(p, "https://browserbench.org/JetStream2.2/", ".benchmark-result")
    motionmark = run_browser_bench(p, "https://browserbench.org/MotionMark1.3/", "#score")

# --- Metadata ---
date = datetime.now().strftime("%Y/%m/%d %H:%M")
computer = os.uname().nodename
cpu = subprocess.getoutput("lscpu | grep 'Model name' | sed 's/Model name:\\s*//'")
gpu = subprocess.getoutput("lspci | grep -i 'vga' | sed 's/.*controller: //'")
ram = subprocess.getoutput("free -h | awk '/Mem:/ {print $2}'")
storage = subprocess.getoutput("lsblk -d -o NAME,SIZE,TYPE | grep disk | awk '{print $1 \" \" $2}'")
distro = subprocess.getoutput("lsb_release -d 2>/dev/null | cut -f2 || grep PRETTY_NAME /etc/os-release | cut -d= -f2 | tr -d '\"'")
shell = os.environ.get("SHELL","bash")
de = os.environ.get("XDG_CURRENT_DESKTOP","Unknown")

# Manual fields
npu = "None"
comp_flags = "(fill manually)"
repo_level = "(fill manually)"
notes = "(fill)"

# --- Append row ---
with open(CSV_FILE, "a", newline="") as f:
    writer = csv.writer(f)
    writer.writerow([
        date,computer,cpu,gpu,npu,ram,storage,comp_flags,
        distro,shell,de,repo_level,
        sevenzip,openssl,ramspeed,
        fio,"","","",
        glmark2,kernel_build,
        speedometer,jetstream,motionmark,notes
    ])

print("✅ Full benchmark suite + browser scores + metadata appended to", CSV_FILE)
