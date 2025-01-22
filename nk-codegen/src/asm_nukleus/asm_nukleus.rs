use super::types::VarType;
use astgen::ast::*;
use std::collections::HashMap;
use crate::asm_nukleus::x86;
use crate::asm_nukleus::arm;
use crate::asm_nukleus::CodegenError;
use std::ptr;
use super::types::RegisterSize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Architecture {
    X86_64,
    ARM64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Register {
    // x86_64 registers
    RAX, RBX, RCX, RDX, RSI, RDI, RBP, RSP, R8, R9, R10, R11, R12, R13, R14, R15,
    // ARM64 registers
    X0, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15,
    X29, X30, // Added ARM64 frame pointer and link register
}

#[derive(Debug)]
pub enum Instruction {
    // Common instructions
    Move(Register, Register),
    MovImm(Register, i64, RegisterSize),
    MovReg(Register, Register, RegisterSize),
    Load(Register, i64),
    Store(Register, Register),
    Add(Register, Register),
    Sub(Register, Register),
    Mul(Register, Register),
    Div(Register, Register),
    Push(Register),
    Pop(Register),
    Call(String),
    Ret,
    
    // Comparison and jumps
    Cmp(Register, Register),
    Jmp(String),
    Je(String),
    Jne(String),
    Jl(String),
    Jg(String),
    Jle(String),
    Jge(String),
    
    // Function-related instructions
    FunctionLabel(String),
    FunctionCall(String, Vec<Register>),
    SaveRegisters(Vec<Register>),
    RestoreRegisters(Vec<Register>),
    
    // Loop-related instructions
    Label(String),
    JumpToLabel(String),
    CompareAndJump {
        cond: JumpCondition,
        reg1: Register,
        reg2: Register,
        label: String,
    },
    
    // Arithmetic with immediate values
    AddImm(Register, i64),
    SubImm(Register, i64),
    MulImm(Register, i64),
    FunctionPrologue,
    FunctionEpilogue,
}

#[derive(Debug, Clone, Copy)]
pub enum JumpCondition {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

pub struct CompiledCode {
    ptr: *mut u8,
    size: usize,
}

impl Drop for CompiledCode {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.ptr as *mut libc::c_void, self.size);
        }
    }
}

impl CompiledCode {
    pub fn as_ptr(&self) -> *const u8 {
        self.ptr
    }

    fn new(code: Vec<u8>) -> Result<Self, CodegenError> {
        unsafe {
            let size = code.len();
            let ptr = libc::mmap(
                ptr::null_mut(),
                size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                -1,
                0,
            ) as *mut u8;

            if ptr.is_null() {
                return Err(CodegenError::MemoryAllocation);
            }

            // Copy the code to the allocated memory
            ptr::copy_nonoverlapping(code.as_ptr(), ptr, size);

            // Make the memory executable
            if libc::mprotect(
                ptr as *mut libc::c_void,
                size,
                libc::PROT_READ | libc::PROT_EXEC,
            ) != 0 {
                // Clean up allocated memory before returning error
                libc::munmap(ptr as *mut libc::c_void, size);
                return Err(CodegenError::MemoryProtection);
            }

            Ok(CompiledCode { ptr, size })
        }
    }
}

pub struct AsmBuilder {
    arch: Architecture,
    instructions: Vec<Instruction>,
    variable_registers: HashMap<String, Register>,
    label_counter: usize,
}

impl AsmBuilder {
    pub fn new(arch: Architecture) -> Self {
        Self {
            arch,
            instructions: Vec::new(),
            variable_registers: HashMap::new(),
            label_counter: 0,
        }
    }

    pub fn compile(&mut self, ast: Vec<AST>) -> Result<CompiledCode, CodegenError> {
        let mut function_table = HashMap::new();
        
        // First pass: collect all function declarations
        for node in &ast {
            if let AST::Statement(ASTstatement::Function { name, args, return_type, .. }) = node {
                function_table.insert(name.clone(), (args.clone(), *return_type));
            }
        }

        // Second pass: compile functions
        for node in ast {
            match node {
                AST::Statement(stmt) => self.compile_statement(stmt, &function_table)?,
                _ => return Err(CodegenError::UnsupportedExpression(format!("Unexpected top-level expression: {:?}", node))),
            }
        }

        self.generate_machine_code()
    }

    fn compile_statement(&mut self, stmt: ASTstatement, function_table: &HashMap<String, (Vec<ASTtypecomp>, ASTtypename)>) -> Result<(), CodegenError> {
        match stmt {
            ASTstatement::Function { name, args, statements, .. } => {
                self.instructions.push(Instruction::FunctionLabel(name.clone()));
                self.emit_function_prologue(&name);
                
                // Handle function arguments
                for (i, arg) in args.iter().enumerate() {
                    if let ASTtypecomp::Argument { identifier, .. } = arg {
                        let reg = self.get_available_register()
                            .map_err(|e| CodegenError::RegisterAllocationError(e.to_string()))?;
                        // Convert identifier to String
                        self.variable_registers.insert(identifier.to_string(), reg);
                        self.instructions.push(Instruction::MovReg(reg, self.get_param_register(i)?, RegisterSize::QWord));
                    }
                }
                
                for stmt in statements {
                    self.compile_ast(stmt, Register::RAX, function_table)?;
                }
                
                self.emit_function_epilogue();
                Ok(())
            },
            ASTstatement::Let { name, value, .. } => {
                let reg = self.get_available_register()
                    .map_err(|e| CodegenError::RegisterAllocationError(e.to_string()))?;
                
                // value is Box<AST>, directly dereference it
                let ast = *value.unwrap();
                self.compile_ast(ast, reg, function_table)?;
                self.variable_registers.insert(name, reg);
                Ok(())
            },
            ASTstatement::Assignment { left, op, right } => {
                if let AST::TypeValue(ASTtypevalue::Identifier(name)) = *left {
                    let dest_reg = *self.variable_registers.get(&name)
                        .ok_or_else(|| CodegenError::UndefinedVariable(name.clone()))?;
                    
                    let temp_reg = self.get_available_register()
                        .map_err(|e| CodegenError::RegisterAllocationError(e.to_string()))?;
                    
                    self.compile_ast(*right, temp_reg, function_table)?;
                    
                    match op {
                        ASTOperator::Assign => self.instructions.push(Instruction::MovReg(dest_reg, temp_reg, RegisterSize::QWord)),
                        ASTOperator::AddAssign => {
                            self.instructions.push(Instruction::Add(dest_reg, temp_reg));
                        },
                        ASTOperator::SubAssign => {
                            self.instructions.push(Instruction::Sub(dest_reg, temp_reg));
                        },
                        ASTOperator::MulAssign => {
                            self.instructions.push(Instruction::Mul(dest_reg, temp_reg));
                        },
                        ASTOperator::DivAssign => {
                            self.instructions.push(Instruction::Div(dest_reg, temp_reg));
                        },
                        _ => return Err(CodegenError::UnsupportedOperator(format!("Unsupported assignment operator: {:?}", op))),
                    }
                    Ok(())
                } else {
                    Err(CodegenError::UnsupportedExpression("Left side of assignment must be an identifier".to_string()))
                }
            },
            ASTstatement::For { start, end, value, statements } => {
                self.compile_for(start, end, value, statements, function_table)
            },
            ASTstatement::Return { value } => {
                let reg = self.get_available_register()
                    .map_err(|e| CodegenError::RegisterAllocationError(e.to_string()))?;
                // value is Box<AST>, directly dereference it
                let ast = *value;
                self.compile_ast(ast, reg, function_table)?;
                self.instructions.push(Instruction::MovReg(Register::RAX, reg, RegisterSize::QWord));
                Ok(())
            },
            _ => Err(CodegenError::UnsupportedStatement(format!("Unsupported statement: {:?}", stmt))),
        }
    }

    fn compile_ast(&mut self, ast: AST, dest_reg: Register, function_table: &HashMap<String, (Vec<ASTtypecomp>, ASTtypename)>) -> Result<(), CodegenError> {
        match ast {
            AST::Statement(stmt) => self.compile_statement(stmt, function_table),
            AST::Logic(logic) => self.compile_logic(logic, dest_reg, function_table),
            AST::TypeValue(value) => self.compile_value(value, dest_reg, function_table),
            _ => Err(CodegenError::UnsupportedExpression("Unsupported AST node".to_string())),
        }
    }

    fn compile_logic(&mut self, logic: ASTlogic, dest_reg: Register, function_table: &HashMap<String, (Vec<ASTtypecomp>, ASTtypename)>) -> Result<(), CodegenError> {
        match logic {
            ASTlogic::BinaryOperation { left, op, right } => {
                let left_reg = self.get_available_register()?;
                
                // Compile left operand
                self.compile_ast(*left, left_reg, function_table)?;
                
                // Compile right operand directly into destination register
                self.compile_ast(*right, dest_reg, function_table)?;

                // Perform operation
                match op {
                    ASTOperator::Add => self.instructions.push(Instruction::Add(dest_reg, left_reg)),
                    ASTOperator::Subtract => self.instructions.push(Instruction::Sub(dest_reg, left_reg)),
                    ASTOperator::Multiply => self.instructions.push(Instruction::Mul(dest_reg, left_reg)),
                    ASTOperator::Divide => self.instructions.push(Instruction::Div(dest_reg, left_reg)),
                    ASTOperator::Equals => {
                        self.instructions.push(Instruction::Cmp(dest_reg, left_reg));
                        self.instructions.push(Instruction::Je(format!("eq_{}", self.label_counter)));
                    },
                    _ => return Err(CodegenError::UnsupportedOperator(format!("Unsupported operator: {:?}", op))),
                }

                // Return the temporary register to the pool
                self.return_register(left_reg);
                Ok(())
            },
        }
    }

    fn compile_value(&mut self, value: ASTtypevalue, dest_reg: Register, function_table: &HashMap<String, (Vec<ASTtypecomp>, ASTtypename)>) -> Result<(), CodegenError> {
        match value {
            ASTtypevalue::I64(n) => {
                self.instructions.push(Instruction::MovImm(dest_reg, n, RegisterSize::QWord));
                Ok(())
            },
            ASTtypevalue::Identifier(name) => {
                if let Some(&src_reg) = self.variable_registers.get(&name) {
                    self.instructions.push(Instruction::MovReg(dest_reg, src_reg, RegisterSize::QWord));
                    Ok(())
                } else {
                    Err(CodegenError::UndefinedVariable(name))
                }
            },
            ASTtypevalue::FunctionCall { name, args } => {
                let (expected_args, _) = function_table.get(&name)
                    .ok_or_else(|| CodegenError::FunctionNotFound(name.clone()))?;
                
                if args.len() != expected_args.len() {
                    return Err(CodegenError::InvalidFunctionArguments(
                        format!("Function {} expects {} arguments, got {}", name, expected_args.len(), args.len())
                    ));
                }

                // Save all currently used registers
                let used_regs = self.get_used_registers();
                self.instructions.push(Instruction::SaveRegisters(used_regs.clone()));

                // Evaluate arguments
                let mut arg_regs = Vec::with_capacity(args.len());
                for arg in args {
                    let reg = self.get_available_register()?;
                    self.compile_ast(arg, reg, function_table)?;
                    arg_regs.push(reg);
                }

                // Call function
                self.instructions.push(Instruction::FunctionCall(name.clone(), arg_regs.clone()));
                
                // Restore saved registers
                self.instructions.push(Instruction::RestoreRegisters(used_regs));
                
                // Move return value to destination register
                self.instructions.push(Instruction::MovReg(dest_reg, Register::RAX, RegisterSize::QWord));
                
                // Return argument registers to the pool
                for reg in arg_regs {
                    self.return_register(reg);
                }
                
                Ok(())
            },
            _ => Err(CodegenError::UnsupportedValueType(format!("Unsupported value type: {:?}", value))),
        }
    }

    fn get_used_registers(&self) -> Vec<Register> {
        self.variable_registers.values().cloned().collect()
    }

    fn get_available_register(&mut self) -> Result<Register, CodegenError> {
        match self.arch {
            Architecture::X86_64 => {
                // Define registers in order of allocation preference
                let registers = [
                    Register::RAX, Register::RCX, Register::RDX, Register::RBX,
                    Register::RSI, Register::RDI, Register::R8, Register::R9,
                    Register::R10, Register::R11, Register::R12, Register::R13,
                    Register::R14, Register::R15
                ];

                // Skip registers that are marked as in use in variable_registers
                for &reg in &registers {
                    if !self.variable_registers.values().any(|&r| r == reg) {
                        return Ok(reg);
                    }
                }
                Err(CodegenError::NoAvailableRegisters)
            },
            Architecture::ARM64 => {
                // Define registers in order of allocation preference
                let registers = [
                    Register::X0, Register::X1, Register::X2, Register::X3,
                    Register::X4, Register::X5, Register::X6, Register::X7,
                    Register::X8, Register::X9, Register::X10, Register::X11,
                    Register::X12, Register::X13, Register::X14, Register::X15
                ];

                // Skip registers that are marked as in use in variable_registers
                for &reg in &registers {
                    if !self.variable_registers.values().any(|&r| r == reg) {
                        return Ok(reg);
                    }
                }
                Err(CodegenError::NoAvailableRegisters)
            }
        }
    }

    fn emit_function_prologue(&mut self, name: &str) {
        match self.arch {
            Architecture::X86_64 => {
                self.instructions.push(Instruction::Push(Register::RBP));
                self.instructions.push(Instruction::MovReg(Register::RBP, Register::RSP, RegisterSize::QWord));
            },
            Architecture::ARM64 => {
                self.instructions.push(Instruction::Push(Register::X29));
                self.instructions.push(Instruction::Push(Register::X30));
            },
        }
    }

    fn emit_function_epilogue(&mut self) {
        match self.arch {
            Architecture::X86_64 => {
                self.instructions.push(Instruction::MovReg(Register::RSP, Register::RBP, RegisterSize::QWord));
                self.instructions.push(Instruction::Pop(Register::RBP));
            },
            Architecture::ARM64 => {
                self.instructions.push(Instruction::Pop(Register::X30));
                self.instructions.push(Instruction::Pop(Register::X29));
            },
        }
        self.instructions.push(Instruction::Ret);
    }

    fn compile_for(&mut self, start: ASTtypevalue, end: ASTtypevalue, value: ASTtypevalue, statements: Vec<AST>, function_table: &HashMap<String, (Vec<ASTtypecomp>, ASTtypename)>) -> Result<(), CodegenError> {
        let loop_start = format!("loop_start_{}", self.label_counter);
        let loop_end = format!("loop_end_{}", self.label_counter);
        self.label_counter += 1;

        // Initialize loop variable
        if let ASTtypevalue::Identifier(var_name) = start {
            let var_reg = *self.variable_registers.get(&var_name)
                .ok_or_else(|| CodegenError::UndefinedVariable(var_name.clone()))?;
            
            // Compare and jump
            self.instructions.push(Instruction::Label(loop_start.clone()));
            let end_reg = self.get_available_register()
                .map_err(|e| CodegenError::RegisterAllocationError(e.to_string()))?;
            
            self.compile_value(end, end_reg, function_table)?;
            self.instructions.push(Instruction::Cmp(var_reg, end_reg));
            self.instructions.push(Instruction::Jg(loop_end.clone()));  // Use Jg for greater than comparison

            // Loop body
            for stmt in statements {
                self.compile_ast(stmt, Register::RAX, function_table)?;
            }

            // Increment
            let step_reg = self.get_available_register()
                .map_err(|e| CodegenError::RegisterAllocationError(e.to_string()))?;
            self.compile_value(value, step_reg, function_table)?;
            self.instructions.push(Instruction::Add(var_reg, step_reg));
            self.instructions.push(Instruction::Jmp(loop_start));
            self.instructions.push(Instruction::Label(loop_end));
            
            Ok(())
        } else {
            Err(CodegenError::UnsupportedExpression("Loop variable must be an identifier".to_string()))
        }
    }

    fn get_param_register(&self, index: usize) -> Result<Register, CodegenError> {
        match (self.arch, index) {
            (Architecture::X86_64, 0) => Ok(Register::RDI),
            (Architecture::X86_64, 1) => Ok(Register::RSI),
            (Architecture::X86_64, 2) => Ok(Register::RDX),
            (Architecture::X86_64, 3) => Ok(Register::RCX),
            (Architecture::ARM64, i) if i < 8 => Ok(Register::X0), // Simplified for now
            _ => Err(CodegenError::InvalidFunctionArguments("Too many function arguments".to_string())),
        }
    }

    fn return_register(&mut self, reg: Register) {
        self.variable_registers.retain(|_, &mut v| v != reg);
    }

    fn generate_machine_code(&self) -> Result<CompiledCode, CodegenError> {
        let machine_code = match self.arch {
            Architecture::X86_64 => {
                let mut generator = x86::X86Generator::new();
                generator.generate(&self.instructions)?
            }
            Architecture::ARM64 => {
                let mut generator = arm::ARMGenerator::new();
                generator.generate(&self.instructions)?
            }
        };

        CompiledCode::new(machine_code)
    }
} 