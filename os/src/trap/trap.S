.altmacro

.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm

.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm

    .section .text
    .globl __alltraps
    .globl __restore
    .align 2

__alltraps:
    # switch from user stack to kernel stack
    csrrw sp, sscratch, sp

    # allocate TrapContext on kernel stack
    addi sp, sp, -34*8

    # save general-purpose registers
    sd x1, 1*8(sp)
    sd x3, 3*8(sp)

    # save x5 ~ x31
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr

    # save sstatus and sepc
    csrr t0, sstatus
    csrr t1, sepc

    sd t0, 32*8(sp)
    sd t1, 33*8(sp)

    # save user stack pointer
    csrr t2, sscratch
    sd t2, 2*8(sp)

    # call trap handler
    mv a0, sp
    call trap_handler

__restore:
    # restore TrapContext
    # 注释掉下面这一行
    # mv sp, a0

    # restore sstatus and sepc
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
