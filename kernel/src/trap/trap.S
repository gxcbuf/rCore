.altmacro # 确保.rept命令能够使用
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm

.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

    .section .text
    .global __alltraps
    .global __restore
    .align 2
__alltraps:
    csrrw sp, sscratch, sp # 交换sp和sscratch的内容
    # 内核栈上分配34*8字节的栈帧，用于保存Trap上下文
    addi sp, sp, -34*8
    # 保存x1
    sd x1, 1*8(sp)
    # 跳过x2
    sd x3, 3*8(sp)
    # 跳过x4(tp), 应用程序不会用到
    # 保存 x5 ~ x31
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    # 将sstatus和sepc读到t0,t1, 然后再保存内核栈
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    # 读取sscrash, 它是进入Trap之前sp的值
    csrr t2, sscratch
    sd t2, 2*8(sp)
    # a0(第一个参数)保存Trap上下文的地址
    mv a0, sp
    call trap_handler

# 恢复被保存在内核栈的寄存器数据
__restore:
    mv sp, a0
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2
    # 恢复通用寄存器
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    # 释放内核栈上的Trap上下文空间
    addi sp, sp, 34*8
    csrrw sp, sscratch, sp
    sret
