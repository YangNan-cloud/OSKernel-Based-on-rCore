# Building
TARGET := riscv64gc-unknown-none-elf
MODE := debug
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin

# Building mode argument
ifeq ($(MODE), release)
	MODE_ARG := --release
endif

# BOARD
BOARD := qemu
BOOTLOADER := default

# KERNEL ENTRY
KERNEL_ENTRY_PA := 0x80200000

QEMUM_ARGS := -machine virt \
    -nographic \
    -bios $(BOOTLOADER) \
    -kernel $(KERNEL_ELF)

GDB_CLIENT := /opt/riscv64-lp64d--musl--bleeding-edge-2024.02-1/bin/riscv64-buildroot-linux-musl-gdb
GDB_ARGS := -ex 'file target/riscv64gc-unknown-none-elf/debug/os' \
    		-ex 'set arch riscv:rv64' \
			-ex 'target remote localhost:1234'

all: env make-user build run

env:
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
	cargo install cargo-binutils
	rustup component add rust-src
	rustup component add llvm-tools-preview

make-user:
	@cd ../user && make clean && make build

build:
	cargo build $(MODE_ARG)

run:
	qemu-system-riscv64 $(QEMUM_ARGS)

clean:
	@cargo clean

gdbserver:
	qemu-system-riscv64 $(QEMUM_ARGS) -s -S

gdbclient:
	$(GDB_CLIENT) $(GDB_ARGS)
