.option norvc
/* no risc-v compress instruction */

.section .boot, "ax",@progbits
.global _start
.global abort

_start:
	lla		sp, kernel_stack_end
	j			machine_start


.bss
.global kernel_stack
kernel_stack:
	.skip 65536
kernel_stack_end:

