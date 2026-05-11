TARGET := riscv64gc-unknown-none-elf
MODE := release

KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin

BOOTLOADER := ../bootloader/opensbi.bin
KERNEL_ENTRY_PA := 0x80200000

build:
	cargo build --release
	rust-objcopy --binary-architecture=riscv64 $(KERNEL_ELF) --strip-all -O binary $(KERNEL_BIN)

run: build
	qemu-system-riscv64 \
	-machine virt \
	-nographic \
	-bios $(BOOTLOADER) \
	-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)

clean:
	cargo clean
