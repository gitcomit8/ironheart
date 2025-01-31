ENTRY(efi_main) /*Entry point of the program*/

SECTIONS
{
    .text ALIGN(4K) : { *(.text) } > RAM  /*.text is the executable code*/
    .rodata ALIGN(4K) : { *(.rodata) } > RAM /*.rodata is the read-only data eg. strings,constatnts*
    .data ALIGN(4K) : { *(.data) } > RAM /*.data is the initialized global data*/
    .bss ALIGN(4K) : { *(.bss) } > RAM /*.bss is the uninitialized global data filled with zeros*/
    .efi_runtime ALIGN(4K) : { *(.efi_runtime) } > RAM /*.efi_runtime is the runtime data that is used by the EFI firmware*/

    /DISCARD/ : {*(.eh_frame)} /* Discard the exception handling section*/
}

MEMORY
{
    RAM (rwx) : ORIGIN = 0x1000000, LENGTH = 0x300000  /* RAM region is of size 16MB starting from 0x1000000*/
}