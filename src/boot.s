.option norvc
/* no risc-v compress instruction */

.section .boot, "ax",@progbits
.global _start
.global abort

_start:
	lla		sp, KERNEL_STACK_END
	j			machine_start

.bss

KERNEL_STACK_START:
	.skip 65536
KERNEL_STACK_END:

.data

KERNEL_HEAP_START:
	.skip 65536
KERNEL_HEAP_END:

.section .rodata

.global KERNEL_STACK_START_ADDR
KERNEL_STACK_START_ADDR:
	.word KERNEL_STACK_START
.global KERNEL_STACK_END_ADDR
KERNEL_STACK_END_ADDR:
	.word KERNEL_STACK_END

.global KERNEL_HEAP_START_ADDR
KERNEL_HEAP_START_ADDR:
	.word KERNEL_HEAP_START
.global KERNEL_HEAP_END_ADDR
KERNEL_HEAP_END_ADDR:
	.word KERNEL_HEAP_END
