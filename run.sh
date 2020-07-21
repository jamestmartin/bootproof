#!/bin/bash
profile=${1:-"debug"}
mkdir -p drive/EFI/Boot
cp "target/x86_64-unknown-uefi/$profile/bootproof.efi" drive/EFI/Boot/BootX64.efi
qemu-system-x86_64 \
    -drive "if=pflash,format=raw,file=/usr/share/OVMF/OVMF_CODE.fd,readonly=on" \
    -drive "if=pflash,format=raw,file=/usr/share/OVMF/OVMF_VARS.fd,readonly=on" \
    -drive "format=raw,file=fat:rw:drive" \
    -nodefaults -cpu host -smp 8 -m 128M -machine "q35,accel=kvm:tcg" \
    -display gtk,gl=on -vga virtio -serial stdio
