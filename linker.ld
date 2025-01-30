ENTRY(efi_main) //Entry point of the program

SECTIONS
{
    .text : { *(.text) } > ROM  //.text is the executable code
    .rodata : { *(.rodata) } > ROM //.rodata is the read-only data eg. strings,constatnts
    .data : { *(.data) } > RAM //.data is the initialized global data
    .bss : { *(.bss) } > RAM //.bss is the uninitialized global data filled with zeros
    .efi_runtime : { *(.efi_runtime) } > RAM //.efi_runtime is the runtime data that is used by the EFI firmware

    /DISCARD/ : {*(.eh_frame)} // Discard the exception handling section
}

MEMORY
{
    RAM (rwx) : ORIGIN = 0x300000, LENGTH = 0x100000  // RAM region is of size 1MB starting from 0x300000
    ROM (rx) : ORIGIN = 0x200000, LENGTH = 0x100000   // ROM region is of size 1MB starting from 0x200000
                                                      // ROM starts from 0x200000 because of the UEFI firmware reserved space
}