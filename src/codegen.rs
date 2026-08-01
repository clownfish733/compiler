use crate::parser::{Function, FunctionMap};

pub enum ParseError {
    NoMain,
}

pub fn gen_code(map: FunctionMap) -> Result<String, ParseError> {
    let main_function = match map.get("main") {
        Some(f) => f,
        None => return Err(ParseError::NoMain),
    };
    let mut code = get_boiler_prefix();

    Ok(code)
}

fn get_boiler_prefix() -> String {
    ".intel_syntax noprefix
    .set SYS_WRITE, 1
    .set SYS_EXIT, 60
    .set STDOUT, 1

    .section .text
    .global _start
_start:
    mov rdi, 8
    mov rax, SYS_EXIT
    syscall
    "
    .to_string()
}
