.option norvc
/* no risc-v compress instruction */

.section .boot, "ax",@progbits
.global _start
.global abort

_start:
	lla		sp, kernel_stack_end
	j			wadachi_start


.bss
.global kernel_stack
kernel_stack:
	.skip 2048
kernel_stack_end:

