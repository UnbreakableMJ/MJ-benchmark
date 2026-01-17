use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Platform {
    DebianLike,
    FedoraLike,
    ArchLike,
    Nix,
    MacOs,
    Windows,
    FreeBsd,
    NetBsd,
    OpenBsd,
    Unknown,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Platform::DebianLike => "Debian-like",
            Platform::FedoraLike => "Fedora-like",
            Platform::ArchLike => "Arch-like",
            Platform::Nix => "Nix",
            Platform::MacOs => "macOS",
            Platform::Windows => "Windows",
            Platform::FreeBsd => "FreeBSD",
            Platform::NetBsd => "NetBSD",
            Platform::OpenBsd => "OpenBSD",
            Platform::Unknown => "Unknown",
        };
        write!(f, "{}", s)
    }
}

pub fn detect_platform() -> Platform {
    let os = std::env::consts::OS;

    match os {
        "linux" => detect_linux(),
        "macos" => Platform::MacOs,
        "windows" => Platform::Windows,
        "freebsd" => Platform::FreeBsd,
        "netbsd" => Platform::NetBsd,
        "openbsd" => Platform::OpenBsd,
        _ => Platform::Unknown,
    }
}

fn detect_linux() -> Platform {
    if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
        let lc = content.to_lowercase();
        if lc.contains("ubuntu") || lc.contains("debian") || lc.contains("linuxmint") {
            Platform::DebianLike
        } else if lc.contains("fedora") || lc.contains("rhel") || lc.contains("centos") {
            Platform::FedoraLike
        } else if lc.contains("arch") || lc.contains("manjaro") {
            Platform::ArchLike
        } else if lc.contains("nixos") {
            Platform::Nix
        } else {
            Platform::Unknown
        }
    } else {
        Platform::Unknown
    }
}