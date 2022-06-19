## 启动内核

### Qemu模拟器
Qemu模拟器启动后，物理内存的起始位置为 `0x80000000`, 所以我们需要将要执行的代码加载至该地址处。

我们的启动流程大致分为三个阶段：
- 第一阶段：Qemu启动后会会将CPU的程序计数器设置为 `0x1000`，所以实际Qemu执行的第一条指令位于此处，
    接着它会执行几条指令，然后跳转到 `0x80000000`, 并进入第二阶段。
- 第二阶段：我们已经通过指令将bootloader(rustsbi-qemu.bin)放到 `0x80000000`位置处，所以
    进入第二阶段时，bootloader就负责接手，然后做一些初始化工作，跳转到 `0x80200000`，进入第三阶段
- 第三阶段：保证内核代码第一条指令位于 `0x80200000`，内核即可接手对计算机的控制。

真实计算机的加电启动流程也大致与上述阶段类似：
- 第一阶段：加电后CPU的PC寄存器被设置为ROM的内部地址，然后开始运行ROM处的软件，主要是做一些CPU的初始化
    将bootloader的代码、数据，从硬盘加载到物理内存，最后跳转到适当的地址，将控制权交给bootloader
- 第二阶段：bootloader同样会做一些初始化的工作，将操作系统从硬盘加载到物理内存，然后跳转到适当地址，
    将控制权交给操作系统。目前的bootloader已经非常复杂，它负责了很多硬件的处理，本质上也是一个操作系统
- 第三阶段：控制权移交给操作系统，由操作系统给我们提供服务。

### 程序内存布局

在源代被编译为可执行文件之后，我们可以根据其功能，将其进行划分，这种划分称之为**段(section)**，不同的段会编译器放到不同的位置上，这就构成了**内存布局**。
```
# 一种典型的内存布局
[low addr] .text --> .rodata --> .data --> .bss --> heap(堆) --> ... <-- stack(栈) [high addr]
```

- .text：代码段，用于存放程序的代码
- .rodata: 全局数据段，用于存放常数或者常量字符串
- .data: 全局数据段，用于存放可修改的全局数据
- .bss: 用于存放未初始化的全局数据，通常由程序加载代为初始化
- heap(堆)：用于存放程序运行时动态分配的数据，向高地址增长
- stack(栈)：用于函数调用上下文的保存与恢复，也用于存储函数的局部变量，向低地址增长

### 编译流程

1. 编译器(Complier) 将每个源文件从高级语言转化为汇编语言，此时仍是ASCII或其它编码的文本文件
2. 汇编器(Assembler) 将上一步每个源文件文本格式的指令转化为机器码，得到一个二进制的**目标文件**
3. 链接器(Linker) 将上一步得到的所以目标文件及一些外部文件链接在一起，得到一个文字的可执行文件

汇编器输出的每个目标文件都有自己独立的内存布局，链接器会将所有的目标文件整合成一个整体的内存布局，这个过程主要做两件事情：

1. 将不同目标文件的section在目标内存布局中重新排布，得到新的`.text, .rodata, .data, .bss, heap, stack`布局。
2. 将符号替换为具体地址。

> 编译后生成的ELF可执行文件，包含一部分metadata, 所以我们需要将其移除才能让内核正常执行
> 使用rust-objcopy --strip-all 可以移除可执行文件的元数据
 