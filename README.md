# MJ-benchmark
Performance system monitoring and benchmarking app

# MJ Benchmarking Ecosystem  
A unified, cross‑platform benchmarking pipeline for Linux, BSD, macOS, and Windows.

This project provides:

- A universal installer for all required tools  
- A device‑spec collector (hardware + software metadata)  
- A benchmark pipeline (PTS + browser + battery health)  
- A unified CSV output compatible with Google Sheets  
- A Google Sheets template with filters, dropdowns, and conditional formatting  
- A single command (`run_bench.sh`) that runs everything  

---

## 📦 Features

### ✔ Cross‑platform support  
- Debian/Ubuntu  
- Fedora/RHEL  
- Arch/Manjaro  
- NixOS / nix-env  
- FreeBSD / NetBSD / OpenBSD  
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
- Battery health (full, design, %, cycles)  
- BIOS key  
- OS + kernel  
- Upgrade options  

### ✔ Output  
One CSV row per run, matching the Google Sheets master matrix.

---

## 🚀 Installation

### Universal installer (Linux / BSD / macOS)

```bash
chmod +x install_mj_bench_stack.sh
./install_mj_bench_stack.sh