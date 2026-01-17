#MJ Benchmark
Steelbore Benchmarking Orchestrator
Created by Mohamed Hammad

MJ Benchmark is a cross‑platform benchmarking orchestrator designed for repeatable, auditable, and automated performance measurement. It collects system specs, runs PTS and browser benchmarks, generates CSV output, and syncs results to Google Sheets and Google Drive — all wrapped in a high‑fidelity Steelbore TUI.

✨ Features
🖥️ Steelbore TUI
A production‑grade terminal interface featuring:
 * Dynamic Visuals: Animated spinners, pulsing cyan glow (Steelbore identity), and animated progress bars.
 * Status Transitions: Animated success states and failure transitions (✖ + pulsing warning red).
 * Vim‑style Navigation: Use hjkl, gg, G, /, n, N for intuitive control.
 * Utility: Live search bar, per‑step elapsed timers, and a clean separation of logs and progress.
 * Performance: Deterministic, non‑blocking animations.

⚙️ Benchmarking Pipeline
MJ Benchmark automates a comprehensive performance suite:
 * Specs Collection: Detailed hardware and OS information.
 * PTS Benchmarks: Phoronix Test Suite integration.
 * Browser Benchmarks: Speedometer, JetStream, and MotionMark.
 * CSV Generation: Local data persistence.
 * Google Sheets Sync: Real-time data logging.
 * Google Drive Upload: Cloud storage for auditability.

🤖 Headless Auto‑Fallback
Automatically detects environments lacking a TTY (CI/CD, SSH without terminal, pipes):
 * TUI mode falls back to a clean CLI mode.
 * Prevents broken output in non-interactive shells.

🌍 Cross‑Platform
 * Linux: Debian, Fedora, Arch, Nix
 * macOS
 * BSD: FreeBSD, NetBSD, OpenBSD
 * Windows

📦 Installation
Install via Cargo:
cargo install mj-benchmark

Or build from source:
git clone https://github.com/UnbreakableMJ/mj-benchmark
cd mj-benchmark
cargo build --release

🚀 Usage
Run full pipeline (TUI mode)
mj-benchmark run \
  --sheet-id <ID> \
  --drive-folder-id <ID> \
  --client-id <ID> \
  --client-secret <SECRET>

Force CLI mode
mj-benchmark run --mode cli ...

System Detection & Planning
# Detect current platform specs
mj-benchmark detect

# Show the installation plan for dependencies
mj-benchmark plan-install

📊 Output
MJ Benchmark produces a standardized output format including:
 * CSV Data: Device specs, PTS results, Browser scores, and Timestamps.
 * Cloud Sync: Automatic uploads to specified Google Sheets and Drive folders.

🧪 TUI Preview
(Add screenshots here)
The interface focuses on high-fidelity feedback:
 * Pulsing cyan glow and real-time logs.
 * Searchable output for debugging long runs.

🔐 License
MJ Benchmark is licensed under the GNU General Public License, version 3 or later (GPL‑3.0‑or‑later).
See the LICENSE file for the full text.

™️ Trademark Notice
The following names are trademarks of Mohamed Hammad:
 * Steelbore
 * S3cure / S3cure me / S3cure us
 * MJ Benchmark
These names may not be used to endorse or promote derivative products, nor used in any way that suggests affiliation or approval, without prior written permission. This trademark clause is fully compatible with GPL‑3.0‑or‑later.

🤝 Contributing
Contributions are welcome! Please open an issue or submit a pull request to help improve the orchestrator.

📧 Contact
For licensing, commercial use, or trademark inquiries, please reach out to Mohamed Hammad.