OUTPUT_FORMAT(elf64-x86-64)
OUTPUT_ARCH(i386:x86-64)

ENTRY(kernel_main)

SECTIONS
{
    . = 0x100000;
    .text   : {
        *(.text.entry)
        *(.text)
    }
    .rodata : { *(.rodata) }
    .data   : { *(.data) }
    . = ALIGN(16);
    __bss_start = .;
    .bss    : { *(.bss) }
    __bss_end = .;
    . = ALIGN(0x1000);
    __stack_end = .;
    .stack  : { . += 0x100000; }
    __stack_start = .;
    __heap_start = .;
}
