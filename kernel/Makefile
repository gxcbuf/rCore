
# 编译后生成的ELF可执行文件，包含一部分metadata, 所以我们需要将其移除才能让内核正常执行
# 使用rust-objcopy --strip-all 可以移除可执行文件的元数据
kernel:
	@cargo build --release && rust-objcopy --strip-all \
		target/riscv64gc-unknown-none-elf/release/kernel \
			-O binary target/riscv64gc-unknown-none-elf/release/kernel.bin


# 启用GDB调试
# -machine 指定计算机名
# -nographic 不需要图形界面，只提供对外输出字符流
# -bios	设置Qemu开机后的bootloader
# -device 将文件加载到指定位置
# -s可以使Qemu启动本地TCP 1234端口等待GDB客户端连接，-S是Qemu在收到GDB请求后再开始执行
gdb:
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios ../bootloader/rustsbi-qemu.bin \
		-device loader,file=target/riscv64gc-unknown-none-elf/release/kernel.bin,addr=0x80200000 \
		-s -S

# 启动一个GDB客户端进行连接
gdb-client:
	@riscv64-unknown-elf-gdb \
    -ex 'file target/riscv64gc-unknown-none-elf/release/kernel' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'

run:
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios ../bootloader/rustsbi-qemu.bin \
		-device loader,file=target/riscv64gc-unknown-none-elf/release/kernel.bin,addr=0x80200000 \
