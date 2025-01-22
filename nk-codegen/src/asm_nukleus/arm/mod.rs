use crate::asm_nukleus::{
    Architecture, AsmBuilder, CodegenError, Instruction, Register, RegisterSize,
};

pub struct ARMGenerator {
    code: Vec<u8>,
}

impl ARMGenerator {
    pub fn new() -> Self {
        Self { code: Vec::new() }
    }

    pub fn generate(&mut self, instructions: &[Instruction]) -> Result<Vec<u8>, CodegenError> {
        for instruction in instructions {
            match instruction {
                Instruction::MovImm(reg, imm, _) => self.encode_mov_imm(*reg, *imm)?,
                Instruction::MovReg(dest, src, _) => self.encode_mov_reg(*dest, *src)?,
                Instruction::Push(reg) => self.encode_push(*reg)?,
                Instruction::Pop(reg) => self.encode_pop(*reg)?,
                Instruction::Add(dest, src) => self.encode_add(*dest, *src)?,
                Instruction::Sub(dest, src) => self.encode_sub(*dest, *src)?,
                _ => return Err(CodegenError::UnsupportedOperation(format!("Unsupported ARM instruction: {:?}", instruction))),
            }
        }
        Ok(self.code.clone())
    }

    fn encode_mov_imm(&mut self, reg: Register, imm: i64) -> Result<(), CodegenError> {
        let reg_code = self.register_code(reg);
        // MOVZ instruction
        let instruction = 0xD2800000 | (reg_code as u32) | (((imm as u32) & 0xFFFF) << 5);
        self.code.extend_from_slice(&instruction.to_le_bytes());
        Ok(())
    }

    fn encode_mov_reg(&mut self, dest: Register, src: Register) -> Result<(), CodegenError> {
        let dest_code = self.register_code(dest);
        let src_code = self.register_code(src);
        // MOV instruction
        let instruction = 0xAA0003E0 | (src_code as u32) << 16 | (dest_code as u32);
        self.code.extend_from_slice(&instruction.to_le_bytes());
        Ok(())
    }

    fn encode_push(&mut self, reg: Register) -> Result<(), CodegenError> {
        let reg_code = self.register_code(reg);
        // STP X29, X30, [SP, #-16]! for frame pointer and link register
        if reg == Register::X29 {
            let instruction: u32 = 0xA9BF7BFD;
            self.code.extend_from_slice(&instruction.to_le_bytes());
        } else {
            // STR Xn, [SP, #-16]!
            let instruction: u32 = 0xF81F0FE0 | (reg_code as u32);
            self.code.extend_from_slice(&instruction.to_le_bytes());
        }
        Ok(())
    }

    fn encode_pop(&mut self, reg: Register) -> Result<(), CodegenError> {
        let reg_code = self.register_code(reg);
        // LDP X29, X30, [SP], #16 for frame pointer and link register
        if reg == Register::X29 {
            let instruction: u32 = 0xA8C17BFD;
            self.code.extend_from_slice(&instruction.to_le_bytes());
        } else {
            // LDR Xn, [SP], #16
            let instruction: u32 = 0xF84107E0 | (reg_code as u32);
            self.code.extend_from_slice(&instruction.to_le_bytes());
        }
        Ok(())
    }

    fn encode_add(&mut self, dest: Register, src: Register) -> Result<(), CodegenError> {
        let dest_code = self.register_code(dest);
        let src_code = self.register_code(src);
        // ADD instruction
        let instruction = 0x8B000000 | (src_code as u32) << 16 | (dest_code as u32) << 5 | dest_code as u32;
        self.code.extend_from_slice(&instruction.to_le_bytes());
        Ok(())
    }

    fn encode_sub(&mut self, dest: Register, src: Register) -> Result<(), CodegenError> {
        let dest_code = self.register_code(dest);
        let src_code = self.register_code(src);
        // SUB instruction
        let instruction = 0xCB000000 | (src_code as u32) << 16 | (dest_code as u32) << 5 | dest_code as u32;
        self.code.extend_from_slice(&instruction.to_le_bytes());
        Ok(())
    }

    fn encode_function_prologue(&mut self) -> Result<(), CodegenError> {
        // STP X29, X30, [SP, #-16]!
        let instruction: u32 = 0xA9BF7BFD;
        self.code.extend_from_slice(&instruction.to_le_bytes());
        // MOV X29, SP
        let instruction: u32 = 0x910003FD;
        self.code.extend_from_slice(&instruction.to_le_bytes());
        Ok(())
    }

    fn encode_function_epilogue(&mut self) -> Result<(), CodegenError> {
        // MOV SP, X29
        let instruction: u32 = 0x910003BF;
        self.code.extend_from_slice(&instruction.to_le_bytes());
        // LDP X29, X30, [SP], #16
        let instruction: u32 = 0xA8C17BFD;
        self.code.extend_from_slice(&instruction.to_le_bytes());
        Ok(())
    }

    fn register_code(&self, reg: Register) -> u8 {
        match reg {
            Register::X0 => 0,
            Register::X1 => 1,
            Register::X2 => 2,
            Register::X3 => 3,
            Register::X4 => 4,
            Register::X5 => 5,
            Register::X6 => 6,
            Register::X7 => 7,
            Register::X8 => 8,
            Register::X9 => 9,
            Register::X10 => 10,
            Register::X11 => 11,
            Register::X12 => 12,
            Register::X13 => 13,
            Register::X14 => 14,
            Register::X15 => 15,
            Register::X29 => 29,
            Register::X30 => 30,
            _ => 0, // Should not happen for ARM64
        }
    }
}
