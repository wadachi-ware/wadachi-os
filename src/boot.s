.option norvc
/* no risc-v compress instruction */

.section .boot, "ax",@progbits
.global _start
.global abort

_start:
	# set pmpcfg pmpaddr
	li		t0, 0b00001111 
	# 7     Lock: false --> memory protection is disable in machine mode
	# 3..=4 Address Maching: TOR --> Top of range
	# 2     Read: true
	# 1     Write: true
	# 0     Execute: true 

	csrw	pmpcfg0, t0
	li		t0, 0xffff0000 # 0 ~ 0xffff0000
	csrw	pmpaddr0, t0
	
	lla		sp, kernel_stack_end
	j			machine_start

.bss
.global kernel_stack
kernel_stack:
	.skip 65536
kernel_stack_end:

