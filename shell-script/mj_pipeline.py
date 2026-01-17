#!/usr/bin/env python3
# MJ Unified Benchmark Pipeline (Benchmark Half)
# Outputs ONE CSV row (no header) with PTS + Browser results

import argparse, json, os, subprocess
from datetime import datetime
from playwright.sync_api import sync_playwright

# --- Parse arguments ---
parser = argparse.ArgumentParser()
parser.add_argument("--output", required=True, help="Output file for CSV row")
args = parser.parse_args()

OUTPUT_FILE = args.output

RUN_NAME = "MJ-core"

# --- Run PTS suite ---
subprocess.run(["phoronix-test-suite", "benchmark", RUN_NAME])

# --- Parse PTS JSON results ---
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

# --- Browser benchmarks ---
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

# --- Notes placeholder ---
notes = "(fill)"

# --- Compose CSV row (benchmark-only) ---
row = f"{sevenzip},{openssl},{ramspeed},{fio},,,,{glmark2},{kernel_build},{speedometer},{jetstream},{motionmark},{notes}"

# Write to output file
with open(OUTPUT_FILE, "w") as f:
    f.write(row)

print(f"✅ Benchmark results written to {OUTPUT_FILE}")