# Building
TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := target/$(TARGET)/$(MODE)/kernel
KERNEL_BIN := $(KERNEL_ELF).bin
DISASM_TMP := target/$(TARGET)/$(MODE)/asm

# Building mode argument
ifeq ($(MODE), release)
	MODE_ARG := --release
endif

# BOARD
BOARD ?= qemu
SBI ?= rustsbi
BOOTLOADER := ../bootloader/$(SBI)-$(BOARD).bin

# KERNEL ENTRY
ifeq ($(BOARD), qemu)
	KERNEL_ENTRY_PA := 0x80200000
endif

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# Disassembly
DISASM ?= -x

.PHONY: build kernel clean run gdb gdbclient

build: $(KERNEL_BIN)

# 编译后生成的ELF可执行文件，包含一部分metadata, 所以我们需要将其移除才能让内核正常执行
# 使用rust-objcopy --strip-all 可以移除可执行文件的元数据
$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

kernel:
	@echo Platform: $(BOARD)
	@cargo build $(MODE_ARG)

clean:
	@cargo clean


run: build
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \

# 启用GDB调试
# -machine 指定计算机名
# -nographic 不需要图形界面，只提供对外输出字符流
# -bios	设置Qemu开机后的bootloader
# -device 将文件加载到指定位置
# -s可以使Qemu启动本地TCP 1234端口等待GDB客户端连接，-S是Qemu在收到GDB请求后再开始执行
gdb: build
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) \
		-s -S

# 启动一个GDB客户端进行连接
gdbclient:
	@riscv64-unknown-elf-gdb \
    -ex 'file $(KERNEL_ELF)' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'
