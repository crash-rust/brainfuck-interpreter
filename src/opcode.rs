use std::collections::HashMap;

// Opcode主要做操作符映射，将枚举成员映射到对应操作符的ASCII值
#[derive(Debug, PartialEq)]
pub enum Opcode {
    //  > 指针加1
    SHR = 0x3E,
    //  < 指针减1
    SHL = 0x3C,
    // + 指针所指字节的值加1
    ADD = 0x2B,
    // - 指针所指字节的值减1
    SUB = 0x2D,
    // . 输出指针所指字节内容（ASCII码）
    PUTCHAR = 0x2E,
    // , 向指针所指的字节输入内容（ASCII码）
    GETCHAR = 0x2C,
    // [ 若指针所指字节的值为零，则向后跳转，跳转到其对应的]的下一个指令处
    LB = 0x5B,
    // ] 若指针所指字节的值不为零，则向前跳转，跳转到其对应的[的下一个指令处
    RB = 0x5D,
}

// 1个解释器去运行代码，接收的代码是从本地的文件中读取的，读取到的内容是u8内容，但是我们需要的是Opcode类型的，所以需要进行转换，将u8转Opcode

// From<T>: 实现From trait，其中T就是想从T类型变成Opcode
impl From<u8> for Opcode {
    fn from(u: u8) -> Self {
        match u {
            0x3E => Opcode::SHR,
            0x3C => Opcode::SHL,
            0x2B => Opcode::ADD,
            0x2D => Opcode::SUB,
            0x2E => Opcode::PUTCHAR,
            0x2C => Opcode::GETCHAR,
            0x5B => Opcode::LB,
            0x5D => Opcode::RB,
            // 使用unreachable程序会报错退出
            // 原因是：u8能表示的ASCII值远比以上Opcode的多，传入进来解释器没法执行也没用，所以拒绝接收直接报错
            _ => unreachable!(),
        }
    }
}

// Into<T>: 实现Into trait， 其中T就是想从Opcode转换的类型
impl Into<u8> for Opcode {
    fn into(self) -> u8 {
        match self {
            Opcode::SHR => 0x3E,
            Opcode::SHL => 0x3C,
            Opcode::ADD => 0x2B,
            Opcode::SUB => 0x2D,
            Opcode::PUTCHAR => 0x2E,
            Opcode::GETCHAR => 0x2C,
            Opcode::LB => 0x5B,
            Opcode::RB => 0x5D,
        }
    }
}

pub struct Code {
    // Opcode指令集
    pub instrs: Vec<Opcode>,
    // 加速bf的执行，bf的跳转是通过[]来执行的，可以通过hashmap记录[]映射
    pub jtable: HashMap<usize, usize>,
}

impl Code {
    pub fn from(data: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        let dict: Vec<u8> = vec![
            Opcode::SHR.into(),
            Opcode::SHL.into(),
            Opcode::ADD.into(),
            Opcode::SUB.into(),
            Opcode::PUTCHAR.into(),
            Opcode::GETCHAR.into(),
            Opcode::LB.into(),
            Opcode::RB.into(),
        ];

        // 取出data代码进行过滤并转换，过滤出dict字典中的这类值，因为这些是可以转成Opcode的u8，然后转换他们成Opcode
        let instrs: Vec<Opcode> = data
            .iter()
            .filter(|x| dict.contains(x))
            .map(|x| Opcode::from(*x))
            .collect();

        let mut jstack: Vec<usize> = Vec::new();
        let mut jtable: HashMap<usize, usize> = HashMap::new();

        for (i, e) in instrs.iter().enumerate() {
            if Opcode::LB == *e {
                jstack.push(i);
            }

            if Opcode::RB == *e {
                let j = jstack.pop().ok_or("pop from empty list")?;
                jtable.insert(j, i);
                jtable.insert(i, j);
            }
        }

        Ok(Code { instrs, jtable })
    }
}
