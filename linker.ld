ENTRY(_start)             /* Entry point symbol (matches your assembly/Rust code) */

SECTIONS {
    . = 0x100000;         /* Load address (1MB mark, common for BIOS bootloaders) */

    .text : {
        *(.text .text.*)  /* All .text sections from input files */
        . = ALIGN(4K);    /* Align to 4KB boundary (useful for paging later) */
    }

    .rodata : {
        *(.rodata .rodata.*)
        . = ALIGN(4K);
    }

    .data : {
        *(.data .data.*)
        . = ALIGN(4K);
    }

    .bss : {
        *(COMMON)         /* Common symbols (uninitialized globals) */
        *(.bss .bss.*)
        . = ALIGN(4K);
    }

    /DISCARD/ : {         /* Discard unnecessary sections */
        *(.comment)
        *(.eh_frame)      /* Rust/C++ exception handling (not needed in no_std) */
    }
}