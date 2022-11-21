use crate::opcode::{Code, Opcode};
use std::io::{stdin, stdout, Read, Write};

#[derive(Debug)]
pub struct Interpreter {
    stack: Vec<u8>, // 表示bf中无限长的纸带
}

impl Default for Interpreter {
    fn default() -> Self {
        Self { stack: vec![0; 1] }
    }
}

impl Interpreter {
    pub fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let code = Code::from(data)?;
        // 不能取data.len()，因为很可能data中还存在不是brainfuck定义的八个字符之外的字符
        // code_len 指令集的长度
        let code_len = code.instrs.len();
        let mut pc = 0; // Program counter 当前程序执行到哪个指令了了
        let mut ps = 0; // Stack Pointer 在stack（无限长纸带）的索引

        // 解释器主循环，无限循环
        loop {
            // 代码执行完了
            if pc >= code_len {
                break;
            }

            match code.instrs[pc] {
                // 如果ps = 0，已经到最左边了，不需要继续往左了，所以就一直固定为0
                Opcode::SHL => ps = if ps == 0 { 0 } else { ps - 1 },
                Opcode::SHR => {
                    ps += 1;
                    // ps+1后，很可能指针会超过stack的，所以应该判断如果事跟stack长度相同，说明越界了，应该主动给stack补格子，也就是加0
                    if ps == self.stack.len() {
                        self.stack.push(0);
                    }
                }
                Opcode::ADD => {
                    self.stack[ps] = self.stack[ps].overflowing_add(1).0;
                }
                Opcode::SUB => {
                    self.stack[ps] = self.stack[ps].overflowing_sub(1).0;
                }
                Opcode::PUTCHAR => {
                    stdout().write_all(&[self.stack[ps]])?;
                }
                Opcode::GETCHAR => {
                    let mut buf: Vec<u8> = vec![0; 1];
                    stdin().read_exact(&mut buf)?;
                    self.stack[ps] = buf[0];
                }
                Opcode::LB => {
                    if self.stack[ps] == 0x00 {
                        pc = code.jtable[&pc];
                    }
                }
                Opcode::RB => {
                    if self.stack[ps] != 0x00 {
                        pc = code.jtable[&pc];
                    }
                }
            }

            pc += 1;
        }

        Ok(())
    }
}
