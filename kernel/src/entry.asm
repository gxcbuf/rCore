# qemu 启动会会从 0x80000000 处加载内核
# 我们将RustSBI放到该地址处，然后RustSBI会跳转至 0x80200000
# kernel.ld 让下面的代码放到 0x80200000处
    .section .text.entry
    .global _start # 定义全局符号
_start:
    la sp, boot_stack_top
    call rust_main

# 用于栈空间
# boot_stack是栈底(低地址)，boot_stack_top是栈顶(高地址)， 栈的增长方向是从高到低
    .section .bss.stack
    .global boot_stack
boot_stack:
    .space 4096 * 16    # 使用64K的空间用于栈空间
    .global boot_stack_top
boot_stack_top:
