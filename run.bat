mkdir esp\EFI\BOOT
copy target\x86_64-unknown-uefi\debug\spacegamex64-rs.efi esp\EFI\BOOT\BOOTX64.EFI
qemu-system-x86_64 -bios DEBUGX64_OVMF.fd -drive format=raw,file=fat:rw:./esp