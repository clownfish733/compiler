use crate::{
    parser::{BinOp, Block, Expr, Function, FunctionMap, FunctionParam, GeneralType, Stmnt, UnOp},
    tokenizer::Type,
};
use std::collections::HashMap;

const PARAM_REGISTERS: &[Register] = &[
    Register {
        U64: "rdi",
        U32: "edi",
        U16: "di",
    },
    Register {
        U64: "rsi",
        U32: "esi",
        U16: "si",
    },
    Register {
        U64: "rdx",
        U32: "edx",
        U16: "dx",
    },
    Register {
        U64: "rcx",
        U32: "ecx",
        U16: "sx",
    },
    Register {
        U64: "r8",
        U32: "r8d",
        U16: "r8w",
    },
    Register {
        U64: "r9",
        U32: "r9d",
        U16: "r9w",
    },
];

#[allow(non_snake_case)]
struct Register {
    U64: &'static str,
    U32: &'static str,
    U16: &'static str,
}

pub enum CodeGenError {
    NoMain,
    VariableNotInstantiated { ident: String },
    VariableInstantiatedTwice { ident: String },
    CompilerError(String),
}

pub fn gen_code(map: FunctionMap) -> Result<String, CodeGenError> {
    if !map
        .0
        .contains_key("main")
    {
        return Err(CodeGenError::NoMain);
    }
    let mut code = get_boiler_prefix();
    code.extend(gen_built_in());
    for (_ident, function) in map.0 {
        code.extend(FunctionCodeGen::generate(function)?);
    }

    code.extend(gen_boiler_suffix());
    Ok(code.join("\n"))
}

fn get_boiler_prefix() -> Vec<String> {
    vec![
        ".intel_syntax noprefix",
        "\t.set SYS_WRITE, 1",
        "\t.set SYS_EXIT, 60",
        "\t.set STDOUT, 1",
        "\t.set ZERO, 48",
        "\t.section .text",
        "\t.global _start",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

fn gen_boiler_suffix() -> Vec<String> {
    vec![
        "_start:",
        "\tpush rbp",
        "\tmov rbp, rsp",
        "\tcall main",
        "\tmov rdi, 8",
        "\tmov rax, SYS_EXIT",
        "\tsyscall",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

fn gen_built_in() -> Vec<String> {
    vec![
        "print:",
        "\tpush rbp",
        "\tmov rbp, rsp",
        "\tsub rsp, 32",
        "\tmov r9, -1",
        "\tmov byte ptr [rbp - 1], 10",
        "\tmov eax, edi",
        ".Lprint_loop:",
        "\txor edx, edx",
        "\tmov ecx, 10",
        "\tdiv ecx",
        "\tdec r9",
        "\tadd dl, ZERO",
        "\tmov byte ptr [rbp + r9], dl",
        "\ttest eax, eax",
        "\tjnz .Lprint_loop",
        "Lprint_end:",
        "\tlea rsi, [rbp + r9]",
        "\tneg r9",
        "\tmov rdx, r9",
        "\tmov rdi, STDOUT",
        "\tmov rax, SYS_WRITE",
        "\tsyscall",
        "\tleave",
        "\tret",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

struct SymbolTable {
    symbols: HashMap<String, (usize, Type)>,
    ofset: usize,
    phantom_ofset: usize,
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self {
            symbols: HashMap::default(),
            ofset: 4,
            phantom_ofset: 0,
        }
    }
}

impl SymbolTable {
    //return ofset for easier init
    //TODO: assume that only type is u32 so delta ofset is always 4
    fn let_var(&mut self, ident: &str, _type: &Type) -> Result<usize, CodeGenError> {
        if self
            .symbols
            .insert(ident.to_string(), (self.ofset, _type.clone()))
            .is_some()
        {
            Err(CodeGenError::VariableInstantiatedTwice {
                ident: ident.to_string(),
            })
        } else {
            let ofset = self.ofset;
            self.ofset += 4;
            Ok(ofset)
        }
    }

    //return ofset and _type (so that appropriate ops can be used)
    fn get_var(&self, ident: &String) -> Result<(usize, Type), CodeGenError> {
        if let Some((ofset, _type)) = self
            .symbols
            .get(ident)
        {
            Ok((*ofset, _type.clone()))
        } else {
            Err(CodeGenError::VariableNotInstantiated {
                ident: ident.to_string(),
            })
        }
    }

    //TODO: assume only u32 so delta ofset is always 4
    fn get_phantom(&mut self, _type: Type) -> usize {
        let phantom_ofset = self.phantom_ofset;
        self.phantom_ofset += 4;
        phantom_ofset + self.ofset
    }

    fn remove_phantom(&mut self, ofset: usize, _type: &Type) -> Result<(), CodeGenError> {
        self.phantom_ofset -= 4;
        if self.phantom_ofset + self.ofset != ofset {
            Err(CodeGenError::CompilerError(
                "Tried to pop from none top of stack".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    fn get_alloc_size(&self) -> usize {
        (self.ofset + 64).next_multiple_of(16)
    }
}

struct FunctionCodeGen {
    lines: Vec<String>,
    symbols: SymbolTable,
    ident: String,
    //for labeling
    while_count: usize,
    if_count: usize,
}

#[allow(unused_variables)]
impl FunctionCodeGen {
    fn new(ident: String) -> Self {
        Self {
            lines: Vec::default(), //reserving space for alloc
            ident,
            symbols: SymbolTable::default(),
            while_count: 0,
            if_count: 0,
        }
    }
    fn push_line(&mut self, line: String) {
        self.lines
            .push(line);
    }
    //return loop, end
    fn get_while_label(&mut self) -> (String, String) {
        let loop_label = format!(".L{}_While_Loop{}", self.ident, self.while_count);
        let end_label = format!(".L{}_While_End{}", self.ident, self.while_count);
        self.while_count += 1;
        (loop_label, end_label)
    }

    fn get_if_label(&mut self) -> String {
        let skip_label = format!(".L{}_If_Skip{}", self.ident, self.if_count);
        self.if_count += 1;
        skip_label
    }

    fn get_else_if_label(&mut self) -> (String, String) {
        let else_label = format!(".L{}_Else{}", self.ident, self.if_count);
        let end_label = format!(".L{}_If_End{}", self.ident, self.if_count);
        self.if_count += 1;
        (else_label, end_label)
    }

    fn generate(function: Function) -> Result<Vec<String>, CodeGenError> {
        let mut generator = Self::new(
            function
                .ident
                .clone(),
        );
        generator.label()?;
        generator.reverse_allocate_stack_space();
        generator.store_args(function.api.params)?;
        generator.generate_block(function.block)?;
        generator.leave_function();
        generator.allocate_stack_space()?;
        Ok(generator.lines)
    }

    fn reverse_allocate_stack_space(&mut self) {
        for _ in 0..3 {
            self.lines
                .push("".to_string())
        }
    }

    fn label(&mut self) -> Result<(), CodeGenError> {
        self.push_line(format!("{}:", self.ident));
        Ok(())
    }

    fn leave_function(&mut self) {
        self.push_line("\tleave".to_string());
        self.push_line("\tret".to_string());
    }

    fn store_args(&mut self, params: Option<Vec<FunctionParam>>) -> Result<(), CodeGenError> {
        if let Some(params) = params {
            for (FunctionParam { ident, _type }, reg) in params
                .iter()
                .zip(PARAM_REGISTERS)
            {
                match _type {
                    Type::U32 => {
                        let ofset = self
                            .symbols
                            .let_var(ident, _type)?;
                        self.push_line(format!("\tmov dword ptr [rbp - {}], {}", ofset, reg.U32))
                    }
                }
            }
        }
        Ok(())
    }

    fn generate_block(&mut self, block: Block) -> Result<(), CodeGenError> {
        for stmnt in block {
            match stmnt {
                Stmnt::Let { ident, _type, init } => {
                    //the phantom logic should leave the result of the init in the instantiated
                    //varaiables segment of the stack
                    let res_ofset = self.generate_expr(init)?;
                    self.symbols
                        .remove_phantom(res_ofset, &_type)?;
                    let ofset = self
                        .symbols
                        .let_var(&ident, &_type)?;
                }
                Stmnt::Assign { ident, init } => {
                    let res_ofset = self.generate_expr(init)?;
                    let (var_ofset, _type) = self
                        .symbols
                        .get_var(&ident)?;
                    self.symbols
                        .remove_phantom(res_ofset, &_type)?;
                    self.push_line(format!("\tmov eax, dword ptr [rbp - {}]", res_ofset));
                    self.push_line(format!("\tmov dword ptr [rbp - {}], eax", var_ofset))
                }
                Stmnt::Return(exp) => {
                    if let Some(exp) = exp {
                        let res_ofset = self.generate_expr(exp)?;
                        self.symbols
                            .remove_phantom(res_ofset, &Type::U32)?;
                        self.push_line(format!("\tmov eax, dword ptr [rbp - {}]", res_ofset));
                    }
                }
                Stmnt::VoidFunction { ident, args } => {
                    if let Some(args) = args {
                        for (arg, reg) in args
                            .iter()
                            .zip(PARAM_REGISTERS)
                        {
                            let res_ofset = self.generate_expr(arg.clone())?;
                            self.symbols
                                .remove_phantom(res_ofset, &Type::U32)?;
                            self.push_line(format!(
                                "\tmov {}, dword ptr [rbp - {}]",
                                reg.U32, res_ofset
                            ));
                        }
                        self.push_line(format!("\tcall {}", ident))
                    }
                }
                Stmnt::While { condition, block } => {
                    let (loop_label, end_label) = self.get_while_label();
                    self.push_line(format!("{}:", loop_label));
                    let res_ofset = self.generate_expr(condition)?;
                    self.symbols
                        .remove_phantom(res_ofset, &Type::U32)?;
                    self.push_line(format!("\tcmp dword ptr [rbp - {}], 0", res_ofset));
                    self.push_line(format!("\tjz {}", end_label));
                    self.generate_block(block)?;
                    self.push_line(format!("\tjmp {}", loop_label));
                    self.push_line(format!("{}:", end_label));
                }
                Stmnt::If {
                    condition,
                    if_block,
                } => {
                    let skip_label = self.get_if_label();
                    let res_ofset = self.generate_expr(condition)?;
                    self.symbols
                        .remove_phantom(res_ofset, &Type::U32)?;
                    self.push_line(format!("\tcmp dword ptr [rbp - {}], 0", res_ofset));
                    self.push_line(format!("\tjz {}", skip_label));
                    self.generate_block(if_block)?;
                    self.push_line(format!("{}:", skip_label));
                }
                Stmnt::IfElse {
                    condition,
                    if_block,
                    else_block,
                } => {
                    let (else_label, end_label) = self.get_else_if_label();
                    let res_ofset = self.generate_expr(condition)?;
                    self.symbols
                        .remove_phantom(res_ofset, &Type::U32)?;
                    self.push_line(format!("\tcmp dword ptr [rbp - {}], 0", res_ofset));
                    self.push_line(format!("\tjz {}", else_label));
                    self.generate_block(if_block)?;
                    self.push_line(format!("\tjmp {}", end_label));
                    self.push_line(format!("{}:", else_label));
                    self.generate_block(else_block)?;
                    self.push_line(format!("{}:", end_label));
                }
            }
        }
        Ok(())
    }

    fn allocate_stack_space(&mut self) -> Result<(), CodeGenError> {
        self.lines[1] = "\tpush rbp".to_string();
        self.lines[2] = "\tmov rbp, rsp".to_string();
        self.lines[3] = format!(
            "\tsub rsp, {}",
            self.symbols
                .get_alloc_size()
        );
        Ok(())
    }

    fn generate_expr(&mut self, expr: Expr) -> Result<usize, CodeGenError> {
        match expr {
            Expr::Lit(GeneralType::Int(n)) => {
                let ofset = self
                    .symbols
                    .get_phantom(Type::U32);
                self.push_line(format!("\tmov dword ptr [rbp - {}], {}", ofset, n));
                Ok(ofset)
            }
            Expr::Binary { lexp, rexp, op } => {
                let l_ofset = self.generate_expr(*lexp)?;
                let r_ofset = self.generate_expr(*rexp)?;
                self.push_line(format!("\tmov eax, dword ptr [rbp - {}]", l_ofset));
                self.push_line(format!("\tmov ebx, dword ptr [rbp - {}]", r_ofset));
                self.symbols
                    .remove_phantom(r_ofset, &Type::U32)?;
                self.symbols
                    .remove_phantom(l_ofset, &Type::U32)?;
                let ofset = self
                    .symbols
                    .get_phantom(Type::U32);
                match op {
                    BinOp::Add => {
                        self.push_line("\tadd eax, ebx".to_string());
                        self.push_line(format!("\tmov dword ptr [rbp - {}], eax", ofset));
                    }
                    BinOp::Sub => {
                        self.push_line("\tsub eax, ebx".to_string());
                        self.push_line(format!("\tmov dword ptr [rbp - {}], eax", ofset));
                    }
                    BinOp::Mul => {
                        self.push_line("\tmul ebx".to_string());
                        self.push_line(format!("\tmov dword ptr [rbp - {}], eax", ofset));
                    }
                    //TODO: ignoring the existance of floor and true div and only handling / as
                    //floor div
                    BinOp::TrueDiv => {
                        self.push_line("\txor edx, edx".to_string());
                        self.push_line("\tdiv ebx".to_string());
                        self.push_line(format!("\tmov dword ptr [rbp - {}], eax", ofset));
                    }
                    BinOp::Mod => {
                        self.push_line("\txor edx, edx".to_string());
                        self.push_line("\tsub eax, ebx".to_string());
                        self.push_line(format!("\tmov dword ptr [rbp - {}], edx", ofset));
                    }
                    //TODO: equiviant for our purposes
                    BinOp::BitOr | BinOp::LogOr => {
                        self.push_line("\tor eax, ebx".to_string());
                        self.push_line(format!("\tmov dword ptr [rbp - {}], eax", ofset));
                    }
                    BinOp::BitAnd | BinOp::LogAnd => {
                        self.push_line("\tand eax, ebx".to_string());
                        self.push_line(format!("\tmov dword ptr [rbp - {}], eax", ofset));
                    }
                    BinOp::EqEq => {
                        self.push_line("\tmov ecx, eax".to_string());
                        self.push_line("\txor eax, eax".to_string());
                        self.push_line("\tcmp ecx, ebx".to_string());
                        self.push_line("\tsete al".to_string());
                        self.push_line(format!("\tmov dword ptr [rbp - {}], eax", ofset));
                    }
                    BinOp::Ne => {
                        self.push_line("\tmov ecx, eax".to_string());
                        self.push_line("\txor eax, eax".to_string());
                        self.push_line("\tcmp ecx, ebx".to_string());
                        self.push_line("\tsetne al".to_string());
                        self.push_line(format!("\tmov dword ptr [rbp - {}], eax", ofset));
                    }
                    BinOp::Lt => {
                        self.push_line("\tmov ecx, eax".to_string());
                        self.push_line("\txor eax, eax".to_string());
                        self.push_line("\tcmp ecx, ebx".to_string());
                        self.push_line("\tsetb al".to_string());
                        self.push_line(format!("\tmov dword ptr [rbp - {}], eax", ofset));
                    }
                    BinOp::Gt => {
                        self.push_line("\tmov ecx, eax".to_string());
                        self.push_line("\txor eax, eax".to_string());
                        self.push_line("\tcmp ecx, ebx".to_string());
                        self.push_line("\tsetg al".to_string());
                        self.push_line(format!("\tmov dword ptr [rbp - {}], eax", ofset));
                    }
                    BinOp::Le => {
                        self.push_line("\tmov ecx, eax".to_string());
                        self.push_line("\txor eax, eax".to_string());
                        self.push_line("\tcmp ecx, ebx".to_string());
                        self.push_line("\tsetbe al".to_string());
                        self.push_line(format!("\tmov dword ptr [rbp - {}], eax", ofset));
                    }
                    BinOp::Ge => {
                        self.push_line("\tmov ecx, eax".to_string());
                        self.push_line("\txor eax, eax".to_string());
                        self.push_line("\tcmp ecx, ebx".to_string());
                        self.push_line("\tsetge al".to_string());
                        self.push_line(format!("\tmov dword ptr [rbp - {}], eax", ofset));
                    }
                    BinOp::FloorDiv => {
                        return Err(CodeGenError::CompilerError(
                            "floor div not implemented for u32".to_string(),
                        ));
                    }
                }
                Ok(ofset)
            }
            Expr::Ident(ident) => {
                let store_ofset = self
                    .symbols
                    .get_phantom(Type::U32);
                let (variable_ofset, _type) = self
                    .symbols
                    .get_var(&ident)?;
                self.push_line(format!("\tmov eax, dword ptr [rbp - {}]", variable_ofset));
                self.push_line(format!("\tmov dword ptr [rbp - {}], eax", store_ofset));
                Ok(store_ofset)
            }
            Expr::Unary { exp, op } => {
                let ofset = self.generate_expr(*exp)?;
                match op {
                    UnOp::Inc => self.push_line(format!("\tinc dword ptr [rbp - {}]", ofset)),
                    UnOp::Dec => self.push_line(format!("\tdec dword ptr [rbp - {}]", ofset)),
                }
                Ok(ofset)
            }
            Expr::Call { ident, args } => {
                if let Some(args) = args {
                    for (arg, reg) in args
                        .iter()
                        .zip(PARAM_REGISTERS)
                    {
                        let arg_ofset = self.generate_expr(arg.clone())?;
                        self.push_line(format!(
                            "\tmov {}, dword ptr [rbp - {}]",
                            reg.U32, &arg_ofset
                        ));
                        self.symbols
                            .remove_phantom(arg_ofset, &Type::U32)?;
                    }
                }

                let ofset = self
                    .symbols
                    .get_phantom(Type::U32);
                self.push_line(format!("\tcall {}", ident));
                self.push_line(format!("\tmov dword ptr [rbp - {}], eax", ofset));
                Ok(ofset)
            }
            Expr::Lit(n) => Err(CodeGenError::CompilerError(
                "floats and bools not implemented".to_string(),
            )),
        }
    }
}
