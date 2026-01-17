# MJ Benchmarking Ecosystem  
A unified, cross‑platform benchmarking and device‑spec collection system with automatic cloud sync to Google Drive and Google Sheets.

This project provides:

- A universal installer for Linux, BSD, macOS, and Windows  
- A device‑spec collector (Bash + PowerShell)  
- A benchmark pipeline (PTS + browser + battery health)  
- A unified CSV schema  
- A Google Sheets–optimized template  
- Automatic sync to Google Drive + Google Sheets  
- A single command (`run_bench`) that runs everything end‑to‑end  

---

## 🚀 Features

### ✔ Cross‑platform support  
- Linux (Debian/Ubuntu, Fedora/RHEL, Arch, NixOS)  
- BSD (FreeBSD, NetBSD, OpenBSD)  
- macOS  
- Windows (PowerShell)

### ✔ Benchmarks included  
- 7‑Zip  
- OpenSSL  
- RAMspeed  
- fio (seq + random)  
- glmark2  
- Linux kernel build  
- Speedometer 2.1  
- JetStream 2.2  
- MotionMark 1.3  

### ✔ Device metadata  
- CPU, GPU, RAM, storage  
- Connectivity, display, build  
- Battery health (Linux)  
- Cameras, biometrics  
- OS + kernel  
- Upgrade options  
- BIOS key  

### ✔ Output  
One CSV row per run, matching the Google Sheets master matrix.

### ✔ Cloud sync  
- Uploads CSV to Google Drive  
- Appends latest row to Google Sheets  

---

## 📦 Installation

### Linux / BSD / macOS  
Use the universal installer:

```bash
chmod +x install_MJ_bench_stack.sh
./install_MN_bench_stack.sh