# Building
TARGET := riscv64gc-unknown-none-elf
MODE := debug
KERNEL_ELF := taget/$(TARGET)/$(MODE)/os
FEATURES := default

QEMUM_ARGS := -machine virt \
    -nographic \
    -bios default \
    -kernel target/riscv64gc-unknown-none-elf/release/os

GDB_CLIENT := /opt/riscv64-lp64d--musl--bleeding-edge-2024.02-1/bin/riscv64-buildroot-linux-musl-gdb
GDB_ARGS := -ex 'file target/riscv64gc-unknown-none-elf/debug/os' \
    		-ex 'set arch riscv:rv64' \
			-ex 'target remote localhost:1234'

build:
	cargo build $(MODE_ARG) --features $(FEATURES)

run:
	qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios default \
    -kernel target/riscv64gc-unknown-none-elf/release/os

clean:
	@cargo clean

gdbserver:
	qemu-system-riscv64 $(QEMUM_ARGS) -s -S

gdbclient:
	$(GDB_CLIENT) $(GDB_ARGS)
