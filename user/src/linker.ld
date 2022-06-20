OUTPUT_ARCH(riscv)
ENTRY(_start)

/* 起始物理地址调整为0x80400000, 应用程序都会被加载到这个物理地址执行 */
BASE_ADDRESS = 0x80400000;

SECTIONS
{
    . = BASE_ADDRESS;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    .data : {
        *(.data .data.*)
        *(.sdata .dsata.*)
    }

    .bss : {
        start_bss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        end_bss = .;
    }

    /DISCARD/ : {
        *(.eh_frame)
        *(.debug*)
    }
}