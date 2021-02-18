.option norvc
/* no risc-v compress instruction */

.section .boot, "ax",@progbits
.global _start
.global abort

_start:
	# set pmpcfg pmpaddr
	li		t0, 0b00001111 # Write Read Execute
	csrw	pmpcfg0, t0
	li		t0, 0xffff0000 # 0 ~ 0xffffffff
	csrw	pmpaddr0, t0
	
	li		t0,	0x10000000
	csrw	sepc, t0

	lla		sp, kernel_stack_end
	j			machine_start

.bss
.global kernel_stack
kernel_stack:
	.skip 65536
kernel_stack_end:

