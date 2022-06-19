OUTPUT_ARCH(riscv) /* 设置平台为riscv */
ENTRY(_start)       /* 设置整个程序的入口点为全局符号_start */
BASE_ADDRESS = 0x80200000; /* 定义一个常量 */

/*
  程序内存布局：从低地址开始往高地址
  依次为 (low addr) .text --> .rodata --> .data ->- .bss --> heap --> ...  <-- stack (high addr)
  .text: 代码段，主要存储程序的汇编代码
  .rodata: 全局数据段，通常存储一些常数或者常量字符串
  .data: 全局数据段，存放一些可修改的全局数据
  .bss: 未初始化数据段，存储那些未初始化的全局数据
  heap(堆): 存放程序运行是动态分配的数据，向高地址增长
  stack(栈): 用于函数调用上下文的保存与恢复，存放函数作用域内的局部变量，向低地址增长
*/
SECTIONS
{
    /* 
      确保 entry.asm / _start 位于0x80200000, qemu会设置为0x80000000,
      我们用来加载OpenSBI,然后OpenSBI会跳转至0x80200000
      
      . 表示当前地址，链接器会从它指向的位置开始往下放置从输入的目标文件中收集来的段

    */
    . = BASE_ADDRESS;
    skernel = .; /* 创建全局符号，并记录起始位置 */

    stext = .;
    .text : {   /* 最终生成可执行文件的一个段的名字‘.text’, 括号内按照放置顺序描述将目标文件放入该段 */
        *(.text.entry)
        *(.text .text.*)
    }

    . = ALIGN(4K);  /* 调整当前位置 */
    etext = .;
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }


    . = ALIGN(4K);
    erodata = .;
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .data.*)
    }

    . = ALIGN(4K);
    edata = .;
    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }

    . = ALIGN(4K);
    ebss = .;
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}