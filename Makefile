

DISK     := esp/EFI/BOOT/BOOTX64.EFI
DISK_DIR := esp/EFI/BOOT
OVMF     := DEBUGX64_OVMF.fd

ifeq ($(OS),Windows_NT)
	CP := copy /Y
	MKDIR = if not exist "$(DISK_DIR)" mkdir "$(DISK_DIR)"
	SOURCE   := target/x86_64-unknown-uefi/debug\spacegamex64-rs.efi
	
else
	CP := cp -f
	MKDIR := mkdir -p $(DISK_DIR)
	SOURCE   := target/x86_64-unknown-uefi/debug/spacegamex64-rs.efi
endif

# 强制重新构建的目标
.PHONY: all run clean

all: run

# 1. 编译 Rust UEFI 程序
$(SOURCE):
	cargo build

# 2. 创建 esp 镜像目录并拷贝 .efi 文件
$(DISK): $(SOURCE)
	$(MKDIR)
	$(CP) "$(SOURCE)" "$(DISK)"

# 3. 下载 OVMF（只在不存在时下载）
$(OVMF):
	curl -L -o $@ https://raw.githubusercontent.com/retrage/edk2-nightly/master/bin/DEBUGX64_OVMF.fd
# 或者你更喜欢 wget：
#	wget -O $@ https://raw.githubusercontent.com/retrage/edk2-nightly/master/bin/DEBUGX64_OVMF.fd

# 4. 创建 fat 镜像并运行（关键！）
run: $(DISK) $(OVMF)
	qemu-system-x86_64 -bios $(OVMF) -drive format=raw,file=fat:rw:./esp

# 可选：清理
clean:
	cargo clean
	rm -rf esp
	rm -f $(OVMF)
