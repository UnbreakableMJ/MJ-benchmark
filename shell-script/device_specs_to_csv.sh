#!/usr/bin/env bash

# Collect system metadata for MJ Benchmark Suite

DATE=$(date +"%Y/%m/%d %H:%M")
COMPUTER=$(hostnamectl | grep "Chassis" || hostname)
CPU=$(lscpu | grep "Model name" | sed 's/Model name:\s*//')
GPU=$(lspci | grep -i 'vga' | sed 's/.*controller: //')
RAM=$(free -h | awk '/Mem:/ {print $2}')
STORAGE=$(lsblk -d -o NAME,SIZE,TYPE | grep disk | awk '{print $1 " " $2}')
DISTRO=$(lsb_release -d 2>/dev/null | cut -f2 || cat /etc/os-release | grep PRETTY_NAME | cut -d= -f2 | tr -d '"')
SHELL=$SHELL
DE=$(echo $XDG_CURRENT_DESKTOP)
# Manual fields
NPU="None"                # Fill manually if present
COMP_FLAGS="(fill manually)" # e.g. -O2 -pipe -march=native
REPO_LEVEL="(fill manually)" # Stable / Testing / Unstable

echo "Date: $DATE"
echo "Computer: $COMPUTER"
echo "CPU: $CPU"
echo "GPU: $GPU"
echo "NPU: $NPU"
echo "RAM: $RAM"
echo "Storage: $STORAGE"
echo "Compilation Flags: $COMP_FLAGS"
echo "Distro: $DISTRO"
echo "Shell: $SHELL"
echo "DE: $DE"
echo "Repo Level: $REPO_LEVEL"
