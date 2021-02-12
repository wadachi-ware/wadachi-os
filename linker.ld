OUTPUT_ARCH("riscv")

ENTRY(_start)

SECTIONS {
	. = 0x80000000;

	.text : {
		*(.boot)
		*(.text .text.*)
	}

	.rodata : {
		*(.rdate .rodata .rodata.*)
	}

	.data : {
		*(.data .data.*)
	}

	.bss : {
		*(.bss bss.*)
	}
}
