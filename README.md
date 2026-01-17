#MJ-benchmark
Mohamed's Benchmarking Ecosystem

A unified, cross‑platform benchmarking and device‑spec collection system with automatic cloud sync to Google Drive and Google Sheets.

This project provides:

- A universal installer for Linux, BSD, macOS, and Windows  
- A device‑spec collector (Bash + PowerShell)  
- A benchmark pipeline (PTS + browser + battery health)  
- A unified CSV schema  
- A Google Sheets–optimized template  
- Automatic sync to Google Drive + Google Sheets  
- A single command (run_bench) that runs everything end‑to‑end  

---

🚀 Features

✔ Cross‑platform support
- Linux (Debian/Ubuntu, Fedora/RHEL, Arch, NixOS)  
- BSD (FreeBSD, NetBSD, OpenBSD)  
- macOS  
- Windows (PowerShell)

✔ Benchmarks included
- 7‑Zip  
- OpenSSL  
- RAMspeed  
- fio (seq + random)  
- glmark2  
- Linux kernel build  
- Speedometer 2.1  
- JetStream 2.2  
- MotionMark 1.3  

✔ Device metadata
- CPU, GPU, RAM, storage  
- Connectivity, display, build  
- Battery health (Linux)  
- Cameras, biometrics  
- OS + kernel  
- Upgrade options  
- BIOS key  

✔ Output
One CSV row per run, matching the Google Sheets master matrix.

✔ Cloud sync
- Uploads CSV to Google Drive  
- Appends latest row to Google Sheets  

---

📦 Installation

Linux / BSD / macOS
Use the universal installer:

chmod +x install_MJ_bench_stack.sh  
./install_MJ_bench_stack.sh

This installs:

- Phoronix Test Suite  
- PHP  
- Python  
- Playwright + browsers  
- Google API libraries  
- Benchmark dependencies  

Windows
Use the PowerShell installer:

Set-ExecutionPolicy Bypass -Scope Process -Force  
.\install_MJ_bench_stack.ps1

This installs:

- Python  
- PHP  
- Git  
- Playwright + browsers  
- Google API libraries  
- Phoronix Test Suite  

---

🔐 Google API Setup (one‑time)

1. Go to Google Cloud Console  
2. Enable:
   - Google Drive API  
   - Google Sheets API  
3. Create OAuth credentials → Desktop App  
4. Download credentials.json  
5. Place it in:

Linux/macOS/BSD:  
~/MJbenchsync/credentials.json

Windows:  
C:\Users\<you>\MJbenchsync\credentials.json

---

🧩 Components

1. Device Specs Collectors

Linux/BSD/macOS:  
device_specs_to_csv.sh --output specstmp.csv

Windows:  
device_specs_to_csv.ps1 --output specstmp.csv

2. Benchmark Pipeline
MJ_pipeline.py --output benchtmp.csv

3. Unified Runner

Linux/BSD/macOS:  
./run_bench.sh

Windows:  
.\run_bench.ps1

---

📄 CSV Schema

The CSV header is:

Brand & Model,Launch Date,Price,CPU & Performance,Codename,CPU Speed,x86-64 Level,GPU,AI & NPU,RAM & Storage,Connectivity,Audio Ports,NFC & Wallet,Battery,Power & Charging,Qi Wireless Charging,Form Factor,Dimensions & Weight,Display,Build & Durability,Cameras,Biometrics & Health,Regional,Software & Updates,Color,Upgrade Options,Ecosystem Lock-in,Wear Detection,Touch Control,Storage Case,Special Features,Official Site,Info Links,BIOS/Boot Key,7-Zip MIPS,OpenSSL MB/s,RAMspeed MB/s,fio Seq Read MB/s,fio Seq Write MB/s,fio Rand Read IOPS,fio Rand Write IOPS,glmark2 Score,Kernel Build Time (s),Speedometer 2.1 Score,JetStream 2.2 Score,MotionMark 1.3 Score,Battery Full Capacity (Wh),Battery Design Capacity (Wh),Battery Health (%),Battery Cycle Count,Notes

---

📊 Google Sheets Template

Includes:

- Filters  
- Dropdowns  
- Conditional formatting  
- Battery health color scale  
- CPU highlighting  
- Blue‑priority color coding  

Use the Apps Script:

setupMJMatrix()

---

☁️ Cloud Sync

The script synctogoogle.py:

- Uploads the CSV to Google Drive  
- Appends the latest row to Google Sheets  

Used automatically by:

run_bench.sh  
run_bench.ps1

---

📂 File Structure

install_MJ_bench_stack.sh  
install_MJ_bench_stack.ps1  
device_specs_to_csv.sh  
device_specs_to_csv.ps1  
MJ_pipeline.py  
run_bench.sh  
run_bench.ps1  
synctogoogle.py  
csv_template.csv  
README.md  

---

🧪 Example Usage

Linux/BSD/macOS:  
./run_bench.sh

Windows:  
.\run_bench.ps1

This will:

1. Collect device specs  
2. Run benchmarks  
3. Merge into CSV  
4. Upload CSV to Google Drive  
5. Append row to Google Sheets  

---

📝 License

This project is licensed under:

- GNU General Public License v3.0 or later (GPL‑3.0+)  
- BSD 3‑Clause License

You may redistribute and/or modify this software under the terms of the GNU GPL as published by the Free Software Foundation, either version 3 of the License or (at your option) any later version.  
https://www.gnu.org/licenses/gpl-3.0.html

---

🤝 Contributions

Pull requests welcome — especially for:

- Additional benchmarks  
- BSD/macOS improvements  
- Windows automation  
- New device categories  
