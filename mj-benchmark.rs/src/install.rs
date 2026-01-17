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

use crate::platform::Platform;
use thiserror::Error;
use std::process::Command;

#[derive(Debug, Error)]
pub enum InstallError {
    #[error("unsupported platform for install")]
    UnsupportedPlatform,
    #[error("command failed: {0}")]
    CommandFailed(String),
}

fn run(cmd: &str, args: &[&str]) -> Result<(), InstallError> {
    println!("> {} {}", cmd, args.join(" "));
    let status = Command::new(cmd).args(args).status();
    match status {
        Ok(s) if s.success() => Ok(()),
        Ok(s) => Err(InstallError::CommandFailed(format!(
            "{} exited with {}",
            cmd, s
        ))),
        Err(e) => Err(InstallError::CommandFailed(format!(
            "failed to start {}: {}",
            cmd, e
        ))),
    }
}

pub fn print_install_plan(platform: Platform) {
    match platform {
        Platform::DebianLike => {
            println!("sudo apt update");
            println!("sudo apt install -y phoronix-test-suite php php-cli php-xml php-json php-curl php-gd php-zip python3 python3-pip");
        }
        Platform::FedoraLike => {
            println!("sudo dnf install -y phoronix-test-suite php php-cli php-xml php-json php-mbstring python3 python3-pip");
        }
        Platform::ArchLike => {
            println!("sudo pacman -S --needed phoronix-test-suite php python python-pip");
        }
        Platform::MacOs => {
            println!("brew install phoronix-test-suite php python");
        }
        Platform::Windows => {
            println!("choco install -y php python git");
            println!("# plus manual PTS setup or custom Windows flow");
        }
        Platform::Nix => {
            println!("# Nix: add PTS, PHP, Python to your system/flake config");
        }
        _ => {
            println!("# Unsupported or unknown platform; please install dependencies manually.");
        }
    }
}

pub fn run_install(platform: Platform) -> Result<(), InstallError> {
    match platform {
        Platform::DebianLike => {
            run("sudo", &["apt", "update"])?;
            run(
                "sudo",
                &[
                    "apt",
                    "install",
                    "-y",
                    "phoronix-test-suite",
                    "php",
                    "php-cli",
                    "php-xml",
                    "php-json",
                    "php-curl",
                    "php-gd",
                    "php-zip",
                    "python3",
                    "python3-pip",
                ],
            )
        }
        Platform::FedoraLike => {
            run(
                "sudo",
                &[
                    "dnf",
                    "install",
                    "-y",
                    "phoronix-test-suite",
                    "php",
                    "php-cli",
                    "php-xml",
                    "php-json",
                    "php-mbstring",
                    "python3",
                    "python3-pip",
                ],
            )
        }
        Platform::ArchLike => {
            run(
                "sudo",
                &[
                    "pacman",
                    "-S",
                    "--needed",
                    "phoronix-test-suite",
                    "php",
                    "python",
                    "python-pip",
                ],
            )
        }
        Platform::MacOs => {
            run("brew", &["install", "phoronix-test-suite", "php", "python"])
        }
        Platform::Windows | Platform::Nix | Platform::FreeBsd | Platform::NetBsd | Platform::OpenBsd | Platform::Unknown => {
            Err(InstallError::UnsupportedPlatform)
        }
    }
}