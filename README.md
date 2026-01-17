#MJ Benchmark
Steelbore Benchmarking Orchestrator  
Created by Mohamed Hammad

MJ Benchmark is a cross‑platform benchmarking orchestrator designed for repeatable, auditable, and automated performance measurement.  
It collects system specs, runs PTS and browser benchmarks, generates CSV output, and syncs results to Google Sheets and Google Drive — all wrapped in a high‑fidelity Steelbore TUI.

The project is licensed under the GNU GPL‑3.0‑or‑later, with protected trademarks (see below).

---

✨ Features

Steelbore TUI
A production‑grade terminal interface with:

- Animated spinner for active steps  
- Pulsing cyan glow (Steelbore identity)  
- Animated progress bar  
- Per‑step elapsed time  
- Animated success transitions  
- Animated failure transitions (✖ + pulsing warning red)  
- Vim‑style navigation (hjkl, gg, G, /, n, N)  
- Live search bar  
- Clean separation of logs and progress  
- Deterministic, non‑blocking animations  

Benchmarking Pipeline
MJ Benchmark runs a full pipeline:

1. Specs collection  
2. PTS benchmarks  
3. Browser benchmarks (Speedometer, JetStream, MotionMark)  
4. CSV generation  
5. Google Sheets sync  
6. Google Drive upload  

Each step is timed, logged, and visually represented in the TUI.

Headless Auto‑Fallback
If the environment lacks a TTY (CI, SSH without terminal, pipes, etc.):

- TUI mode automatically falls back to CLI mode  
- No broken output  
- No user intervention required  

Cross‑Platform
Supports:

- Linux (Debian, Fedora, Arch, Nix)  
- macOS  
- FreeBSD / NetBSD / OpenBSD  
- Windows  

---

📦 Installation

`bash
cargo install mj-benchmark
`

Or build from source:

`bash
git clone https://github.com/yourname/mj-benchmark
cd mj-benchmark
cargo build --release
`

---

🚀 Usage

Run full pipeline (TUI mode)

`bash
mj-benchmark run \
  --sheet-id <ID> \
  --drive-folder-id <ID> \
  --client-id <ID> \
  --client-secret <SECRET>
`

Force CLI mode

`bash
mj-benchmark run --mode cli ...
`

Detect platform

`bash
mj-benchmark detect
`

Show install plan

`bash
mj-benchmark plan-install
`

---

📊 Output

MJ Benchmark produces:

- A CSV row with:
  - Device specs  
  - PTS results  
  - Browser benchmark scores  
  - Timestamp  
- Automatic upload to:
  - Google Sheets  
  - Google Drive  

---

🧪 TUI Preview

(Add screenshots when ready.)

The TUI includes:

- Animated progress indicators  
- Pulsing cyan glow  
- Failure animations  
- Real‑time logs  
- Searchable output  

---

🔐 License

MJ Benchmark is licensed under the:

GNU General Public License, version 3 or later (GPL‑3.0‑or‑later)

See the full license text in:

- LICENSE

---

™ Trademark Notice

The following names are trademarks of Mohamed Hammad:

- Steelbore  
- S3cure  
- S3cure me  
- S3cure us  
- MJ Benchmark

These names may not be used to endorse or promote derivative products, nor used in any way that suggests affiliation or approval, without prior written permission.

This trademark clause is fully compatible with GPL‑3.0‑or‑later.

---

🤝 Contributing

Contributions are welcome!  
Please open an issue or submit a pull request.

---

📧 Contact

For licensing, commercial use, or trademark inquiries:  
Mohamed Hammad