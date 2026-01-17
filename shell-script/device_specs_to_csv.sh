#!/usr/bin/env bash
# MJ Device Spec Auto-Collector → CSV Row Output
# Outputs ONE CSV row (no header), ready for pipeline merging

set -euo pipefail

# --- Parse arguments ---
OUTPUT_FILE=""
while [[ $# -gt 0 ]]; do
    case "$1" in
        --output)
            OUTPUT_FILE="$2"
            shift 2
            ;;
        *)
            echo "Unknown argument: $1"
            exit 1
            ;;
    esac
done

if [[ -z "$OUTPUT_FILE" ]]; then
    echo "Usage: $0 --output <file>"
    exit 1
fi

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
X86_LEVEL=$(get "lscpu | grep -i 'x86-64' | awk '{print \$2}'")

GPU=$(get "lspci | grep -i 'vga' | sed 's/.*controller: //'")

RAM=$(get "free -h | awk '/Mem:/ {print \$2}'")
STORAGE=$(get "lsblk -d -o NAME,SIZE,TYPE | grep disk | awk '{print \$1 \" \" \$2}' | paste -sd '; '")

CONNECTIVITY=$(get "nmcli -t -f WIFI g")
CONNECTIVITY+="; $(get "lsusb | grep -i 'usb' | head -n 1")"

AUDIO_PORTS=$(get "amixer info | grep -i card")

BATTERY=$(get "upower -i $(upower -e | grep BAT) | grep -E 'energy-full:|energy-full-design:' | xargs")
POWER_SUPPLY=$(get "cat /sys/class/power_supply/AC/online 2>/dev/null")

DIMENSIONS=$(get "cat /sys/devices/virtual/dmi/id/chassis_type")
DISPLAY=$(get "xrandr --current | grep '*' | awk '{print \$1}' | head -n 1")

BUILD=$(get "cat /sys/devices/virtual/dmi/id/chassis_vendor")
BIOMETRICS=$(get "lsusb | grep -i 'fingerprint'")

DISTRO=$(get "grep PRETTY_NAME /etc/os-release | cut -d= -f2 | tr -d '\"'")
KERNEL=$(get "uname -r")

UPGRADE_OPTIONS=$(get "lsblk -o NAME,TYPE | grep disk")

BIOS_KEY=""  # Not detectable automatically

# --- Manual / non-detectable fields (left blank) ---
LAUNCH_DATE=""
PRICE=""
AI_NPU=""
NFC=""
QI=""
FORM_FACTOR=""
CAMERAS=""
REGIONAL=""
COLOR=""
ECOSYSTEM=""
WEAR_DETECTION=""
TOUCH_CONTROL=""
STORAGE_CASE=""
SPECIAL=""
OFFICIAL=""
LINKS=""

# --- Compose CSV row in EXACT schema order ---
ROW="$BRAND_MODEL,$LAUNCH_DATE,$PRICE,$CPU,$CODENAME,$CPU_SPEED,$X86_LEVEL,$GPU,$AI_NPU,$RAM / $STORAGE,$CONNECTIVITY,$AUDIO_PORTS,$NFC,$BATTERY,$POWER_SUPPLY,$QI,$FORM_FACTOR,$DIMENSIONS,$DISPLAY,$BUILD,$CAMERAS,$BIOMETRICS,$REGIONAL,$DISTRO ($KERNEL),$COLOR,$UPGRADE_OPTIONS,$ECOSYSTEM,$WEAR_DETECTION,$TOUCH_CONTROL,$STORAGE_CASE,$SPECIAL,$OFFICIAL,$LINKS,$BIOS_KEY"

# Write to output file
echo "$ROW" > "$OUTPUT_FILE"

echo "Device specs row written to $OUTPUT_FILE"