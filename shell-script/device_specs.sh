#!/usr/bin/env bash
# Mohamed Device Spec Auto-Collector
# Collects as many fields as possible automatically for your master CSV schema

set -euo pipefail

# Helper: safe command execution
get() {
    CMD="$1"
    OUTPUT=$(bash -c "$CMD" 2>/dev/null | head -n 1 | xargs)
    [ -z "$OUTPUT" ] && echo "" || echo "$OUTPUT"
}

# --- Auto-detected fields ---
BRAND_MODEL=$(get "hostnamectl | grep 'Model' | cut -d: -f2")
[ -z "$BRAND_MODEL" ] && BRAND_MODEL=$(get "cat /sys/devices/virtual/dmi/id/product_name")

CPU=$(get "lscpu | grep 'Model name' | sed 's/Model name:\\s*//'")
CPU_SPEED=$(get "lscpu | grep 'CPU max MHz' | awk '{printf \"%.2f GHz\", \$4/1000}'")
CPU_SPEED=${CPU_SPEED:-$(get "lscpu | grep 'CPU MHz' | awk '{printf \"%.2f GHz\", \$3/1000}'")}

CODENAME=$(get "lscpu | grep 'Vendor ID' | awk '{print \$3}'")
X86_LEVEL=$(get "lscpu | grep 'x86-64' | awk '{print \$2}'")

GPU=$(get "lspci | grep -i 'vga' | sed 's/.*controller: //'")

RAM=$(get "free -h | awk '/Mem:/ {print \$2}'")
STORAGE=$(get "lsblk -d -o NAME,SIZE,TYPE | grep disk | awk '{print \$1 \" \" \$2}' | paste -sd '; '")

CONNECTIVITY=$(get "nmcli -t -f WIFI g")
CONNECTIVITY+="; $(get "lsusb | grep -i 'usb' | head -n 1")"

AUDIO_PORTS=$(get "amixer info | grep -i card")
NFC=""  # Not detectable on Linux PCs

BATTERY=$(get "upower -i /org/freedesktop/UPower/devices/battery_BAT0 | grep -E 'energy-full|capacity' | xargs")
POWER_SUPPLY=$(get "cat /sys/class/power_supply/AC/online 2>/dev/null")

DIMENSIONS=$(get "cat /sys/devices/virtual/dmi/id/chassis_type")
WEIGHT=""  # Not detectable

DISPLAY=$(get "xrandr --current | grep '*' | awk '{print \$1}' | head -n 1")

BUILD=$(get "cat /sys/devices/virtual/dmi/id/chassis_vendor")
CAMERAS=""  # Not applicable for PCs

BIOMETRICS=$(get "lsusb | grep -i 'fingerprint'")
REGIONAL=""  # Not detectable

DISTRO=$(get "grep PRETTY_NAME /etc/os-release | cut -d= -f2 | tr -d '\"'")
SOFTWARE_UPDATES=$(get "uname -r")

COLOR=""  # Not detectable
UPGRADE_OPTIONS=$(get "lsblk -o NAME,TYPE | grep disk")
ECOSYSTEM=""  # Not applicable for PCs

WEAR_DETECTION=""
TOUCH_CONTROL=""
STORAGE_CASE=""
SPECIAL_FEATURES=""

OFFICIAL_SITE=""
INFO_LINKS=""
BIOS_KEY=$(get "grep -i 'boot' /sys/firmware/efi/efivars 2>/dev/null")

# --- Output in your CSV order ---
echo "Brand & Model: $BRAND_MODEL"
echo "Launch Date: "
echo "Price: "
echo "CPU & Performance: $CPU"
echo "Codename: $CODENAME"
echo "CPU Speed: $CPU_SPEED"
echo "x86-64 Level: $X86_LEVEL"
echo "GPU: $GPU"
echo "AI & NPU: "
echo "RAM & Storage: $RAM / $STORAGE"
echo "Connectivity: $CONNECTIVITY"
echo "Audio Ports: $AUDIO_PORTS"
echo "NFC & Wallet: "
echo "Battery: $BATTERY"
echo "Power & Charging: $POWER_SUPPLY"
echo "Qi Wireless Charging: "
echo "Form Factor: "
echo "Dimensions & Weight: $DIMENSIONS"
echo "Display: $DISPLAY"
echo "Build & Durability: $BUILD"
echo "Cameras: $CAMERAS"
echo "Biometrics & Health: $BIOMETRICS"
echo "Regional: $REGIONAL"
echo "Software & Updates: $DISTRO ($SOFTWARE_UPDATES)"
echo "Color: $COLOR"
echo "Upgrade Options: $UPGRADE_OPTIONS"
echo "Ecosystem Lock-in: $ECOSYSTEM"
echo "Wear Detection: $WEAR_DETECTION"
echo "Touch Control: $TOUCH_CONTROL"
echo "Storage Case: $STORAGE_CASE"
echo "Special Features: $SPECIAL_FEATURES"
echo "Official Site: $OFFICIAL_SITE"
echo "Info Links: $INFO_LINKS"
echo "BIOS/Boot Key: $BIOS_KEY"
