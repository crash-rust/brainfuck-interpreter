## BrainFuck解释器

### 实验概述

本实验依照brainfuck语言定义，通过学习理解brainfuck的相关定义，以及相关的brainfuck知识利用rust实现brainfuck这门语言的解释器。

### 实验目的

- 通过rust学习编译相关知识
- 通过实现简单的解释器熟悉rust当中的知识概念
  - `From<T>` `Into<T>`两个常用Trait
  - `enum`的另一种类ts方式的枚举
  - `unreachable!`宏的使用
  - 处理数字范围溢出的方式：`overflowing_add`,`overflowing_sub`

### brainfuck介绍

Brainfuck，简称BF，是一种极小化的程序语言，由Urban Müller在1993年创造。这种语言基于一个简单的机器模型。这个机器除了指令以外，还包括：一个以字节为单位、已初始化为零的数组、一个指向该数组的指针（开始时指向数组的第一个字节）、以及用于输入输出的两个字节流。

下面是这八种状态的描述，其中每个状态由一个[字符](https://zh.m.wikipedia.org/wiki/字符)标识：

| 字符 |                             含义                             |
| :--: | :----------------------------------------------------------: |
| `>`  |                           指针加一                           |
| `<`  |                           指针减一                           |
| `+`  |                     指针所指字节的值加一                     |
| `-`  |                     指针所指字节的值减一                     |
| `.`  | 输出指针所指字节内容（[ASCII码](https://zh.m.wikipedia.org/wiki/ASCII码)） |
| `,`  |             向指针所指的字节输入内容（ASCII码）              |
| `[`  | 若指针所指字节的值为零，则向后跳转，跳转到其对应的`]`的下一个指令处 |
| `]`  | 若指针所指字节的值不为零，则向前跳转，跳转到其对应的`[`的下一个指令处 |

- 中文链接：https://zh.m.wikipedia.org/zh-cn/Brainfuck
- 英文链接：https://en.wikipedia.org/wiki/Brainfuck
### 使用brainfuck解释器

- 使用解释器
  `cargo run --example run_interpreter --release  ./brainfuck/sierpinski.bf`

  `./brainfuck/sierpinski.bf`可以替换成`brainfuck文件夹里的任何一个bf后缀的文件`
  
- 效果
  ```shell
                                  *
                                 * *
                                *   *
                               * * * *
                              *       *
                             * *     * *
                            *   *   *   *
                           * * * * * * * *
                          *               *
                         * *             * *
                        *   *           *   *
                       * * * *         * * * *
                      *       *       *       *
                     * *     * *     * *     * *
                    *   *   *   *   *   *   *   *
                   * * * * * * * * * * * * * * * *
                  *                               *
                 * *                             * *
                *   *                           *   *
               * * * *                         * * * *
              *       *                       *       *
             * *     * *                     * *     * *
            *   *   *   *                   *   *   *   *
           * * * * * * * *                 * * * * * * * *
          *               *               *               *
         * *             * *             * *             * *
        *   *           *   *           *   *           *   *
       * * * *         * * * *         * * * *         * * * *
      *       *       *       *       *       *       *       *
     * *     * *     * *     * *     * *     * *     * *     * *
    *   *   *   *   *   *   *   *   *   *   *   *   *   *   *   *
   * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
  ```