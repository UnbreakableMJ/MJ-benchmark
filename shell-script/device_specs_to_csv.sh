#!/usr/bin/env bash

# MJ Benchmark Suite Auto-Logger
# Collect metadata + parse PTS results + append into CSV

CSV_FILE="$HOME/MJ_benchmarks.csv"
RUN_NAME=$1   # pass the PTS run name as argument

# Ensure CSV has headers if new
if [ ! -f "$CSV_FILE" ]; then
  echo "Date,Computer,CPU,GPU,NPU,RAM,Storage,Compilation Flags,Distro,Shell,DE,Repo Level,7-Zip MIPS,OpenSSL MB/s,RAMspeed MB/s,fio Seq Read MB/s,fio Seq Write MB/s,fio Rand Read IOPS,fio Rand Write IOPS,glmark2 Score,Kernel Build Time (s),Speedometer 2.1 Score,JetStream 2.2 Score,MotionMark 1.3 Score,Notes" > "$CSV_FILE"
fi

# Metadata
DATE=$(date +"%Y/%m/%d %H:%M")
COMPUTER=$(hostname)
CPU=$(lscpu | grep "Model name" | sed 's/Model name:\s*//')
GPU=$(lspci | grep -i 'vga' | sed 's/.*controller: //')
RAM=$(free -h | awk '/Mem:/ {print $2}')
STORAGE=$(lsblk -d -o NAME,SIZE,TYPE | grep disk | awk '{print $1 " " $2}')
DISTRO=$(lsb_release -d 2>/dev/null | cut -f2 || grep PRETTY_NAME /etc/os-release | cut -d= -f2 | tr -d '"')
SHELL=$SHELL
DE=$XDG_CURRENT_DESKTOP

# Manual fields
NPU="None"
COMP_FLAGS="(fill manually)"
REPO_LEVEL="(fill manually)"

# Parse PTS JSON results
RESULT_DIR="$HOME/.phoronix-test-suite/test-results/$RUN_NAME"
RESULT_JSON="$RESULT_DIR/results.json"

SEVENZIP=$(jq -r '.Results[] | select(.Identifier=="pts/compress-7zip") | .Result' $RESULT_JSON)
OPENSSL=$(jq -r '.Results[] | select(.Identifier=="pts/openssl") | .Result' $RESULT_JSON)
RAMSPEED=$(jq -r '.Results[] | select(.Identifier=="pts/ramspeed") | .Result' $RESULT_JSON)
FIO_SEQ_READ=$(jq -r '.Results[] | select(.Identifier=="pts/fio") | .Result' $RESULT_JSON)
GLMARK2=$(jq -r '.Results[] | select(.Identifier=="pts/glmark2") | .Result' $RESULT_JSON)
KERNEL_BUILD=$(jq -r '.Results[] | select(.Identifier=="pts/build-linux-kernel") | .Result' $RESULT_JSON)

# Browser tests (manual entry for now)
SPEEDOMETER="(fill)"
JETSTREAM="(fill)"
MOTIONMARK="(fill)"
NOTES="(fill)"

# Append row
echo "$DATE,$COMPUTER,$CPU,$GPU,$NPU,$RAM,$STORAGE,$COMP_FLAGS,$DISTRO,$SHELL,$DE,$REPO_LEVEL,$SEVENZIP,$OPENSSL,$RAMSPEED,$FIO_SEQ_READ,,,$GLMARK2,$KERNEL_BUILD,$SPEEDOMETER,$JETSTREAM,$MOTIONMARK,$NOTES" >> "$CSV_FILE"

echo "✅ Benchmark metadata + PTS results appended to $CSV_FILE"
