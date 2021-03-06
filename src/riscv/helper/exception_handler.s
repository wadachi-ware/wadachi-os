.option norvc
.macro SAVE_REGISTERS_TO_USER_STACK
        addi sp, sp, -124

        sw ra, 0(sp)
        sw sp, 4(sp)
        sw gp, 8(sp)
        sw tp, 12(sp)
        sw t0, 16(sp)
        sw t1, 20(sp)
        sw t2, 24(sp)
        sw s0, 28(sp)
        sw s1, 32(sp)
        sw a0, 36(sp)
        sw a1, 40(sp)
        sw a2, 44(sp)
        sw a3, 48(sp)
        sw a4, 52(sp)
        sw a5, 56(sp)
        sw a6, 60(sp)
        sw a7, 64(sp)
        sw s2, 68(sp)
        sw s3, 72(sp)
        sw s4, 76(sp)
        sw s5, 80(sp)
        sw s6, 84(sp)
        sw s7, 88(sp)
        sw s8, 92(sp)
        sw s9, 96(sp)
        sw s10, 100(sp)
        sw s11, 104(sp)
        sw t3, 108(sp)
        sw t4, 112(sp)
        sw t5, 116(sp)
        sw t6, 120(sp)
.endm

.macro LOAD_REGISTERS_FROM_USER_STACK
        lw ra, 0(sp)
        lw sp, 4(sp)
        lw gp, 8(sp)
        lw tp, 12(sp)
        lw t0, 16(sp)
        lw t1, 20(sp)
        lw t2, 24(sp)
        lw s0, 28(sp)
        lw s1, 32(sp)
        lw a0, 36(sp)
        lw a1, 40(sp)
        lw a2, 44(sp)
        lw a3, 48(sp)
        lw a4, 52(sp)
        lw a5, 56(sp)
        lw a6, 60(sp)
        lw a7, 64(sp)
        lw s2, 68(sp)
        lw s3, 72(sp)
        lw s4, 76(sp)
        lw s5, 80(sp)
        lw s6, 84(sp)
        lw s7, 88(sp)
        lw s8, 92(sp)
        lw s9, 96(sp)
        lw s10, 100(sp)
        lw s11, 104(sp)
        lw t3, 108(sp)
        lw t4, 112(sp)
        lw t5, 116(sp)
        lw t6, 120(sp)

        addi sp, sp, 124
.endm

.data

.global HANDLER_POINTER
HANDLER_POINTER:
		.word 0

.section .text, "ax",@progbits

.global test_exception_handler
test_exception_handler:
	.align 2
	SAVE_REGISTERS_TO_USER_STACK
	
	lla	t0, HANDLER_POINTER
	lw	t0, 0(t0)
	jalr	ra, t0, 0

	LOAD_REGISTERS_FROM_USER_STACK

	mret



