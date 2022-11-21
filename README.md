## BrainFuck解释器

### brainfuck介绍
- 中文链接：https://zh.m.wikipedia.org/zh-cn/Brainfuck
- 英文链接：https://en.wikipedia.org/wiki/Brainfuck
### 使用brainfuck解释器

- 使用解释器
  `cargo run --example run_interpreter --release  ./brainfuck/sierpinski.bf`

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