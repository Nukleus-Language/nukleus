use core::arch::asm;
use std::collections::HashMap;
use crate::asm_nukleus::{
    Architecture, AsmBuilder, Instruction, Register,
    asm_nukleus::JumpCondition,
};
use crate::asm_nukleus::types::{CodegenError, RegisterSize};

pub struct X86Generator {
    code: Vec<u8>,
    labels: HashMap<String, usize>,          // Maps label names to positions in code
    pending_labels: Vec<(String, usize)>,    // Labels that need resolution
    used_registers: Vec<Register>,
    symbols: HashMap<String, usize>,         // For function symbols
}

impl X86Generator {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            labels: HashMap::new(),
            pending_labels: Vec::new(),
            used_registers: Vec::new(),
            symbols: HashMap::new(),
        }
    }

    pub fn generate(&mut self, instructions: &[Instruction]) -> Result<Vec<u8>, X86Error> {
        for instruction in instructions {
            match instruction {
                Instruction::MovImm(reg, imm, size) => self.encode_mov_imm(*reg, *imm, *size)?,
                Instruction::MovReg(dest, src, size) => self.encode_mov_reg(*dest, *src, *size)?,
                Instruction::Push(reg) => self.encode_push(*reg)?,
                Instruction::Pop(reg) => self.encode_pop(*reg)?,
                Instruction::Add(dest, src) => self.encode_add(*dest, *src)?,
                Instruction::Sub(dest, src) => self.encode_sub(*dest, *src)?,
                Instruction::Call(label) => self.encode_call(label)?,
                Instruction::Ret => self.encode_ret()?,
                Instruction::Jmp(label) => self.encode_jmp(label)?,
                Instruction::FunctionLabel(name) => self.encode_function_label(name)?,
                Instruction::Mul(dest, src) => self.encode_mul(*dest, *src)?,
                Instruction::Label(name) => self.encode_label(name)?,
                Instruction::Cmp(dest, src) => self.encode_cmp(*dest, *src)?,
                Instruction::Je(label) => self.encode_conditional_jump(JumpCondition::Equal, label)?,
                Instruction::Jne(label) => self.encode_conditional_jump(JumpCondition::NotEqual, label)?,
                Instruction::Jg(label) => self.encode_conditional_jump(JumpCondition::Greater, label)?,
                Instruction::Jl(label) => self.encode_conditional_jump(JumpCondition::Less, label)?,
                Instruction::Jle(label) => self.encode_conditional_jump(JumpCondition::LessEqual, label)?,
                Instruction::Jge(label) => self.encode_conditional_jump(JumpCondition::GreaterEqual, label)?,
                _ => return Err(X86Error::UnsupportedOperation(
                    format!("Unsupported x86 instruction: {:?}", instruction)
                )),
            }
        }
        self.resolve_labels()?;
        Ok(self.code.clone())
    }

    fn encode_mov_imm(&mut self, reg: Register, imm: i64, size: RegisterSize) -> Result<(), X86Error> {
        self.code.push(0x48); // REX.W prefix
        self.code.push(0xB8 + self.register_code(reg)?);
        self.code.extend_from_slice(&imm.to_le_bytes());
        Ok(())
    }

    fn encode_mov_reg(&mut self, dest: Register, src: Register, size: RegisterSize) -> Result<(), X86Error> {
        self.code.push(0x48); // REX.W prefix for 64-bit operands
        self.code.push(0x89); // MOV r/m64, r64
        
        let src_code = self.register_code(src)?;
        let dest_code = self.register_code(dest)?;
        // Construct ModR/M byte: 0b11 (mod) << 6 | src_reg << 3 | dest_reg
        self.code.push(0xC0 | (src_code << 3) | dest_code);
        Ok(())
    }

    fn encode_push(&mut self, reg: Register) -> Result<(), X86Error> {
        let reg_code = self.register_code(reg)?;
        if reg_code >= 8 {
            // REX.W prefix for extended registers (R8-R15)
            self.code.push(0x41);
        }
        self.code.push(0x50 + (reg_code & 7));
        Ok(())
    }

    fn encode_pop(&mut self, reg: Register) -> Result<(), X86Error> {
        let reg_code = self.register_code(reg)?;
        if reg_code >= 8 {
            // REX.W prefix for extended registers (R8-R15)
            self.code.push(0x41);
        }
        self.code.push(0x58 + (reg_code & 7));
        Ok(())
    }

    fn encode_add(&mut self, dest: Register, src: Register) -> Result<(), X86Error> {
        self.code.push(0x48); // REX.W prefix
        self.code.push(0x01);
        self.code.push(0xC0 + (self.register_code(src)? << 3) + self.register_code(dest)?);
        Ok(())
    }

    fn encode_sub(&mut self, dest: Register, src: Register) -> Result<(), X86Error> {
        self.code.push(0x48); // REX.W prefix
        self.code.push(0x29);
        self.code.push(0xC0 + (self.register_code(src)? << 3) + self.register_code(dest)?);
        Ok(())
    }

    fn encode_call(&mut self, label: &str) -> Result<(), X86Error> {
        // For now, we'll implement a simple direct call
        // call instruction (E8 + 32-bit relative offset)
        self.code.push(0xE8);
        // Placeholder for 32-bit relative offset
        self.code.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
        Ok(())
    }

    fn encode_jmp(&mut self, label: &str) -> Result<(), X86Error> {
        // Unconditional jump
        self.code.push(0xE9); // JMP rel32
        // Store the current position for later resolution
        self.pending_labels.push((label.to_string(), self.code.len()));
        // Add placeholder for 32-bit offset
        self.code.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
        Ok(())
    }

    fn encode_ret(&mut self) -> Result<(), X86Error> {
        self.code.push(0xC3);
        Ok(())
    }

    fn encode_function_label(&mut self, name: &str) -> Result<(), X86Error> {
        // Function alignment - align to 16 bytes
        while self.code.len() % 16 != 0 {
            self.code.push(0x90); // NOP padding
        }
        
        // Store the current position in the code for symbol resolution
        self.symbols.insert(name.to_string(), self.code.len());
        
        Ok(())
    }

    fn encode_label(&mut self, name: &str) -> Result<(), X86Error> {
        // Store the current position in the code for label resolution
        self.labels.insert(name.to_string(), self.code.len());
        Ok(())
    }

    fn register_code(&self, reg: Register) -> Result<u8, X86Error> {
        match reg {
            Register::RAX => Ok(0),
            Register::RCX => Ok(1),
            Register::RDX => Ok(2),
            Register::RBX => Ok(3),
            Register::RSP => Ok(4),
            Register::RBP => Ok(5),
            Register::RSI => Ok(6),
            Register::RDI => Ok(7),
            Register::R8 => Ok(8),
            Register::R9 => Ok(9),
            Register::R10 => Ok(10),
            Register::R11 => Ok(11),
            Register::R12 => Ok(12),
            Register::R13 => Ok(13),
            Register::R14 => Ok(14),
            Register::R15 => Ok(15),
            _ => Err(X86Error::InvalidRegister(format!("Invalid x86_64 register: {:?}", reg))),
        }
    }

    fn encode_function_prologue(&mut self) -> Result<(), X86Error> {
        unsafe {
            asm!(
                "push rbp",
                "mov rbp, rsp",
                options(nomem)
            );
        }
        Ok(())
    }

    fn encode_function_epilogue(&mut self) -> Result<(), X86Error> {
        unsafe {
            asm!(
                "mov rsp, rbp",
                "pop rbp",
                "ret",
                options(nomem)
            );
        }
        Ok(())
    }

    fn encode_function_call(&mut self, _name: &str, args: &[Register]) -> Result<(), X86Error> {
        // System V AMD64 ABI calling convention
        let arg_regs = [Register::RDI, Register::RSI, Register::RDX, Register::RCX, Register::R8, Register::R9];
        
        // Save caller-saved registers
        for reg in &[Register::RAX, Register::RCX, Register::RDX, Register::RSI, Register::RDI, Register::R8, Register::R9, Register::R10, Register::R11] {
            self.encode_push(*reg)?;
        }

        // Move arguments to the appropriate registers
        for (i, &reg) in args.iter().enumerate() {
            if i >= arg_regs.len() {
                return Err(X86Error::InvalidFunctionArguments(
                    "Too many arguments for System V AMD64 ABI".to_string()
                ));
            }
            self.encode_mov_reg(arg_regs[i], reg, RegisterSize::QWord)?;
        }

        // Align stack to 16 bytes (System V AMD64 ABI requirement)
        self.code.extend_from_slice(&[0x48, 0x83, 0xE4, 0xF0]); // and rsp, -16

        // Call instruction placeholder
        self.code.extend_from_slice(&[0xE8, 0x00, 0x00, 0x00, 0x00]);

        // Restore caller-saved registers in reverse order
        for reg in [Register::R11, Register::R10, Register::R9, Register::R8, Register::RDI, Register::RSI, Register::RDX, Register::RCX, Register::RAX].iter() {
            self.encode_pop(*reg)?;
        }

        Ok(())
    }

    fn resolve_labels(&mut self) -> Result<(), X86Error> {
        let pending = std::mem::take(&mut self.pending_labels);
        for (label, offset) in pending {
            let target = self.labels.get(&label)
                .ok_or_else(|| X86Error::InvalidJumpTarget(label.clone()))?;
            
            let target_offset = i32::try_from(*target)
                .map_err(|_| X86Error::OffsetOutOfRange(label.clone()))?;
            let current_offset = i32::try_from(offset + 4)
                .map_err(|_| X86Error::OffsetOutOfRange(label.clone()))?;
            
            let relative_offset = target_offset.checked_sub(current_offset)
                .ok_or_else(|| X86Error::OffsetOutOfRange(label.clone()))?;
            
            let bytes = relative_offset.to_le_bytes();
            
            if offset + 4 > self.code.len() {
                return Err(X86Error::InvalidCodeOffset(label));
            }
            
            self.code[offset..offset + 4].copy_from_slice(&bytes);
        }
        Ok(())
    }

    fn get_asm_reg(&self, reg: Register) -> Result<&'static str, X86Error> {
        match reg {
            Register::RAX => Ok("rax"),
            Register::RBX => Ok("rbx"),
            Register::RCX => Ok("rcx"),
            Register::RDX => Ok("rdx"),
            Register::RSI => Ok("rsi"),
            Register::RDI => Ok("rdi"),
            Register::RBP => Ok("rbp"),
            Register::RSP => Ok("rsp"),
            Register::R8  => Ok("r8"),
            Register::R9  => Ok("r9"),
            Register::R10 => Ok("r10"),
            Register::R11 => Ok("r11"),
            Register::R12 => Ok("r12"),
            Register::R13 => Ok("r13"),
            Register::R14 => Ok("r14"),
            Register::R15 => Ok("r15"),
            _ => Err(X86Error::InvalidRegister(format!("Invalid x86_64 register: {:?}", reg))),
        }
    }

    fn get_available_registers(&self) -> Vec<Register> {
        let all_registers = vec![
            Register::RAX, // Return value
            Register::RDI, // 1st argument
            Register::RSI, // 2nd argument
            Register::RDX, // 3rd argument
            Register::RCX, // 4th argument
            Register::R8,  // 5th argument
            Register::R9,  // 6th argument
            Register::R10, // Caller-saved
            Register::R11, // Caller-saved
            Register::RBX, // Callee-saved
            Register::R12, // Callee-saved
            Register::R13, // Callee-saved
            Register::R14, // Callee-saved
            Register::R15, // Callee-saved
        ];

        all_registers
            .into_iter()
            .filter(|reg| !self.used_registers.contains(reg))
            .collect()
    }

    fn allocate_register(&mut self) -> Result<Register, X86Error> {
        let available = self.get_available_registers();
        if let Some(reg) = available.first() {
            self.used_registers.push(*reg);
            Ok(*reg)
        } else {
            Err(X86Error::RegisterAllocationError("No available registers".to_string()))
        }
    }

    fn deallocate_register(&mut self, reg: Register) {
        if let Some(pos) = self.used_registers.iter().position(|&r| r == reg) {
            self.used_registers.remove(pos);
        }
    }

    fn encode_mul(&mut self, dest: Register, src: Register) -> Result<(), X86Error> {
        // REX.W prefix for 64-bit operands
        self.code.push(0x48);
        
        // If source or destination uses extended registers (R8-R15)
        let last_idx = self.code.len() - 1;
        if self.register_code(src)? >= 8 || self.register_code(dest)? >= 8 {
            self.code[last_idx] |= 0x44;
        }
        
        // IMUL instruction opcode
        self.code.push(0x0F);
        self.code.push(0xAF);
        
        // ModR/M byte: 0b11 (mod) << 6 | dest_reg << 3 | src_reg
        self.code.push(0xC0 | (self.register_code(dest)? << 3) | self.register_code(src)?);
        
        Ok(())
    }

    fn encode_conditional_jump(&mut self, condition: JumpCondition, label: &str) -> Result<(), X86Error> {
        // First byte for conditional jumps
        self.code.push(0x0F);
        
        // Second byte depends on the condition
        let opcode = match condition {
            JumpCondition::Equal => 0x84,         // JE/JZ
            JumpCondition::NotEqual => 0x85,      // JNE/JNZ
            JumpCondition::Less => 0x8C,          // JL/JNGE
            JumpCondition::LessEqual => 0x8E,     // JLE/JNG
            JumpCondition::Greater => 0x8F,       // JG/JNLE
            JumpCondition::GreaterEqual => 0x8D,  // JGE/JNL
        };
        self.code.push(opcode);

        // Store current position for label resolution
        let current_pos = self.code.len();
        self.pending_labels.push((label.to_string(), current_pos));
        
        // Add placeholder for 32-bit offset
        self.code.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
        
        Ok(())
    }

    fn encode_cmp(&mut self, dest: Register, src: Register) -> Result<(), X86Error> {
        // REX.W prefix for 64-bit operands
        self.code.push(0x48);
        
        // If source or destination uses extended registers (R8-R15)
        let last_idx = self.code.len() - 1;
        if self.register_code(src)? >= 8 || self.register_code(dest)? >= 8 {
            self.code[last_idx] |= 0x44;
        }
        
        // CMP instruction opcode
        self.code.push(0x39);
        
        // ModR/M byte: 0b11 (mod) << 6 | src_reg << 3 | dest_reg
        self.code.push(0xC0 | (self.register_code(src)? << 3) | self.register_code(dest)?);
        
        Ok(())
    }
}

#[derive(Debug)]
pub struct RegisterAllocation {
    pub register: Register,
    pub is_caller_saved: bool,
}

impl RegisterAllocation {
    pub fn new(register: Register, is_caller_saved: bool) -> Self {
        Self {
            register,
            is_caller_saved,
        }
    }
}

#[derive(Debug)]
pub enum X86Error {
    OffsetOutOfRange(String),
    InvalidCodeOffset(String),
    InvalidJumpTarget(String),
    UnsupportedOperation(String),
    InvalidFunctionArguments(String),
    RegisterAllocationError(String),
    InvalidRegister(String),  // New variant
}

impl From<X86Error> for CodegenError {
    fn from(error: X86Error) -> Self {
        match error {
            X86Error::OffsetOutOfRange(msg) => CodegenError::ArchitectureError(format!("Offset out of range: {}", msg)),
            X86Error::InvalidCodeOffset(msg) => CodegenError::ArchitectureError(format!("Invalid code offset: {}", msg)),
            X86Error::InvalidJumpTarget(msg) => CodegenError::ArchitectureError(format!("Invalid jump target: {}", msg)),
            X86Error::UnsupportedOperation(msg) => CodegenError::UnsupportedOperation(msg),
            X86Error::InvalidFunctionArguments(msg) => CodegenError::InvalidFunctionArguments(msg),
            X86Error::RegisterAllocationError(msg) => CodegenError::RegisterAllocationError(msg),
            X86Error::InvalidRegister(msg) => CodegenError::ArchitectureError(msg),
        }
    }
}
