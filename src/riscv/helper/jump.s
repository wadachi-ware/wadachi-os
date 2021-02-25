.option norvc

.global setjmp
.global longjmp

.section text, "ax",@progbits

.macro STORE_REGISTER reg_name index
	sw			\reg_name, (\index * 4)(a0)
.endm

.macro LOAD_REGISTER reg_name index
	lw			\reg_name, (\index * 4)(a0)
.endm

# REGISTER STRUCTOR:
#   s0
#   s1
#   ...
#   s11
#   sp
#   ra

setjmp:
	# setjmp(jmp_buf buffer);
	STORE_REGISTER s0, 0
	STORE_REGISTER s1, 1
	STORE_REGISTER s2, 2
	STORE_REGISTER s3, 3
	STORE_REGISTER s4, 4
	STORE_REGISTER s5, 5
	STORE_REGISTER s6, 6
	STORE_REGISTER s7, 7
	STORE_REGISTER s8, 8
	STORE_REGISTER s9, 9
	STORE_REGISTER s10, 10
	STORE_REGISTER s11, 11
	STORE_REGISTER sp, 12
	STORE_REGISTER ra, 13
	
	li				a0, 0

	ret

longjmp:
	# longjmp(jmp_buf buffer, int ret_code);
	LOAD_REGISTER s0, 0
	LOAD_REGISTER s1, 1
	LOAD_REGISTER s2, 2
	LOAD_REGISTER s3, 3
	LOAD_REGISTER s4, 4
	LOAD_REGISTER s5, 5
	LOAD_REGISTER s6, 6
	LOAD_REGISTER s7, 7
	LOAD_REGISTER s8, 8
	LOAD_REGISTER s9, 9
	LOAD_REGISTER s10, 10
	LOAD_REGISTER s11, 11
	LOAD_REGISTER sp, 12
	LOAD_REGISTER ra, 13
	
	# a0: return value
	# a1: argument
	# see riscv abi ducumentation: https://riscv.org/wp-content/uploads/2015/01/riscv-calling.pdf
	seqz			a0, a1
	add			a0, a0, a1

	ret


