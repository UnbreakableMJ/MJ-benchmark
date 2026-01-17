#!/usr/bin/env bash
# Universal installer for MJ's stack
# Targets: Debian/Ubuntu, Fedora/RHEL, Arch, Nix, FreeBSD, NetBSD, OpenBSD, macOS

set -euo pipefail

# ---- Helpers ---------------------------------------------------------

have_cmd() {
    command -v "$1" >/dev/null 2>&1
}

need_root() {
    if [ "$EUID" -ne 0 ]; then
        if have_cmd sudo; then
            echo "Re-running with sudo..."
            exec sudo -E "$0" "$@"
        else
            echo "Please run as root."
            exit 1
        fi
    fi
}

# ---- OS / Distro detection ------------------------------------------

OS="$(uname -s)"

if [ "$OS" = "Linux" ] && [ -f /etc/os-release ]; then
    . /etc/os-release
    DISTRO_ID="${ID:-unknown}"
else
    DISTRO_ID="unknown"
fi

echo "Detected OS: $OS  Distro: $DISTRO_ID"

# ---- Install per platform -------------------------------------------

install_debian_ubuntu() {
    need_root "$@"
    apt update
    apt install -y phoronix-test-suite php php-curl php-gd php-zip \
        python3 python3-pip sysbench fio stress-ng cpu-x hardinfo mbw iozone3 glmark2-data xonotic
}

install_fedora_rhel() {
    need_root "$@"
    dnf install -y phoronix-test-suite php php-cli php-common php-gd php-zip php-curl \
        python3 python3-pip sysbench fio stress-ng cpu-x hardinfo mbw iozone glmark2 xonotic
}

install_arch() {
    # assumes paru already installed
    paru -S --needed \
        phoronix-test-suite php php-curl php-gd php-zip \
        python python-pip \
        sysbench fio stress-ng cpu-x hardinfo mbw iozone glmark2 xonotic
}

install_nixos_nix() {
    echo "Installing via nix-env (user profile)..."
    nix-env -iA \
        nixpkgs.phoronix-test-suite \
        nixpkgs.php \
        nixpkgs.phpExtensions.curl \
        nixpkgs.phpExtensions.gd \
        nixpkgs.phpExtensions.zip \
        nixpkgs.python3 \
        nixpkgs.python3Packages.pip \
        nixpkgs.sysbench \
        nixpkgs.fio \
        nixpkgs.stress-ng \
        nixpkgs.cpu-x \
        nixpkgs.hardinfo \
        nixpkgs.mbw \
        nixpkgs.iozone \
        nixpkgs.glmark2 \
        nixpkgs.xonotic
}

install_freebsd() {
    need_root "$@"
    pkg update -f
    pkg install -y phoronix-test-suite php php-curl php-gd php-zip \
        python3 py39-pip sysbench fio stress-ng cpu-x hardinfo mbw iozone glmark2
    # Xonotic may be missing or named differently; skip if unavailable
}

install_netbsd() {
    need_root "$@"
    pkgin update
    pkgin install -y phoronix-test-suite python311 py311-pip sysbench fio
    # Extra tools (cpu-x, hardinfo, glmark2, xonotic) may not be available in pkgsrc
}

install_openbsd() {
    need_root "$@"
    pkg_add php php-gd php-curl php-zip wget python3 py3-pip
    if ! have_cmd phoronix-test-suite; then
        echo "Installing Phoronix Test Suite from upstream tarball..."
        cd /tmp
        wget https://phoronix-test-suite.com/releases/phoronix-test-suite-10.8.4.tar.gz
        tar xzf phoronix-test-suite-10.8.4.tar.gz
        cd phoronix-test-suite
        ./install-sh
    fi
    # Many Linux-only tools (cpu-x, hardinfo, glmark2, xonotic) are not relevant here
}

install_macos() {
    if ! have_cmd brew; then
        echo "Homebrew not found. Install from https://brew.sh/ and rerun."
        exit 1
    fi
    brew update
    brew install phoronix-test-suite php python sysbench fio stress-ng \
        glmark2
    # cpu-x, hardinfo, xonotic may be missing; okay for now
}

# ---- Main dispatcher -------------------------------------------------

case "$OS" in
    Linux)
        case "$DISTRO_ID" in
            debian|ubuntu|linuxmint|pop|zorin)
                install_debian_ubuntu "$@"
                ;;
            fedora|rhel|centos|rocky|alma)
                install_fedora_rhel "$@"
                ;;
            arch|manjaro|endeavouros|cachyos)
                install_arch "$@"
                ;;
            nixos)
                install_nixos_nix "$@"
                ;;
            *)
                echo "Unknown or unsupported Linux distro ID: $DISTRO_ID"
                echo "You can still use: nix-shell -p phoronix-test-suite php"
                exit 1
                ;;
        esac
        ;;
    FreeBSD)
        install_freebsd "$@"
        ;;
    NetBSD)
        install_netbsd "$@"
        ;;
    OpenBSD)
        install_openbsd "$@"
        ;;
    Darwin)
        install_macos "$@"
        ;;
    *)
        echo "Unsupported OS: $OS"
        echo "For Windows, use the PowerShell installer instead."
        exit 1
        ;;
esac

# ---- Python Playwright setup (common) -------------------------------

if have_cmd python3; then
    echo "Installing Playwright Python package and browsers..."
    python3 -m pip install --user playwright
    python3 -m playwright install
else
    echo "python3 not found; skipping Playwright install."
fi

echo "✅ MJ benchmark stack installation complete."