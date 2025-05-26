use std::collections::HashMap;
use util_lib::lang_type::{LangTypeEnum, number_type::Int, number_type::Float, char_type::Char};
mod print_util;
use print_util::nasm_print_function;

/// Translate SIL (Structured Intermediate Language) to Linux NASM assembly code.
pub fn translate_to_linux_nasm(sil: &str) -> String {
    let mut nasm_code = String::new();
    let mut variables: HashMap<String, LangTypeEnum> = HashMap::new(); // 中間言語変数を管理するマップ

    nasm_code.push_str("section .text\n");
    nasm_code.push_str(nasm_print_function());
    nasm_code.push_str("_start:\n");

    for line in sil.lines() {
        let trimmed = line.trim();
        let parts: Vec<&str> = trimmed.split_whitespace().collect();

        match parts.get(0).map(|s| *s) {
            // INT型変数の宣言
            Some("INT") => {
                if let Some(var_name) = parts.get(1) {
                    variables.insert(
                        var_name.to_string(),
                        LangTypeEnum::Int {
                            value: Int { value: 0 }, // 初期値は0
                            name: var_name.to_string(), // 変数名を保持
                        },
                    );
                }
            }
            // FLOAT型変数の宣言
            Some("FLOAT") => {
                if let Some(var_name) = parts.get(1) {
                    variables.insert(
                        var_name.to_string(),
                        LangTypeEnum::Float {
                            value: Float { value: 0.0 }, // 初期値は0.0
                            name: var_name.to_string(), // 変数名を保持
                        },
                    );
                }
            }
            // CHAR型変数の宣言
            Some("CHAR") => {
                if let Some(var_name) = parts.get(1) {
                    variables.insert(
                        var_name.to_string(),
                        LangTypeEnum::Char {
                            value: Char { value: '\0' }, // 初期値はnull文字
                            name: var_name.to_string(), // 変数名を保持
                        },
                    );
                }
            }
            // 配列型変数の宣言
            Some("Array") => {
                if parts.len() >= 4 {
                    let var_name = parts[3];
                    let array_size = parts[2]
                        .trim_start_matches('<')
                        .trim_end_matches('>')
                        .parse::<usize>()
                        .unwrap_or(1);
                    variables.insert(
                        var_name.to_string(),
                        LangTypeEnum::Array {
                            value: Box::new(LangTypeEnum::Int {
                                value: Int { value: array_size as i64 },
                                name: var_name.to_string(),
                            }),
                            name: var_name.to_string(),
                        },
                    );
                }
            }
            Some("PUSH") => {
                let value = parts.get(1).unwrap_or(&"0");
                nasm_code.push_str(&format!("    push {}\n", value));
            }
            Some("ADD") => {
                nasm_code.push_str("    pop rax\n    pop rbx\n    add rax, rbx\n    push rax\n");
            }
            Some("SUB") => {
                nasm_code.push_str("    pop rbx\n    pop rax\n    sub rax, rbx\n    push rax\n");
            }
            Some("MUL") => {
                nasm_code.push_str("    pop rax\n    pop rbx\n    imul rax, rbx\n    push rax\n");
            }
            Some("DIV") => {
                nasm_code.push_str("    pop rbx\n    pop rax\n    cqo\n    idiv rbx\n    push rax\n");
            }
            Some("STORE") => {
                let var = parts.get(1).unwrap_or(&"unknown");
                nasm_code.push_str(&format!("    pop [{}]\n", var));
            }
            Some("LOAD") => {
                let var = parts.get(1).unwrap_or(&"unknown");
                nasm_code.push_str(&format!("    push [{}]\n", var));
            }
            Some("JMP") => {
                let label = parts.get(1).unwrap_or(&"unknown");
                nasm_code.push_str(&format!("    jmp {}\n", label));
            }
            Some("JMPIF") => {
                let label = parts.get(1).unwrap_or(&"unknown");
                nasm_code.push_str("    pop rax\n    cmp rax, 1\n");
                nasm_code.push_str(&format!("    je {}\n", label));
            }
            Some("CALL") => {
                let func = parts.get(1).unwrap_or(&"unknown");
                nasm_code.push_str(&format!("    call {}\n", func));
            }
            Some(instruction) if instruction.ends_with(":") => {
                nasm_code.push_str(&format!("{}:\n", instruction.trim_end_matches(':')));
            }
            Some(instruction) => {
                nasm_code.push_str(&format!("    ;; Unhandled instruction: {}\n", instruction));
            }
            None => {}
        }
    }

    // プログラム終了コードを追加
    nasm_code.push_str("\n    mov rax, 60         ; syscall: exit\n");
    nasm_code.push_str("    xor rdi, rdi        ; status = 0\n");
    nasm_code.push_str("    syscall\n");

    // .bssセクションで変数を確保
    nasm_code.push_str("\nsection .bss\n");
    for (var_name, var_type) in variables {
        match var_type {
            LangTypeEnum::Int { .. } => nasm_code.push_str(&format!("    {} resq 1\n", var_name)),
            LangTypeEnum::Float { .. } => nasm_code.push_str(&format!("    {} resq 1\n", var_name)),
            LangTypeEnum::Char { .. } => nasm_code.push_str(&format!("    {} resb 1\n", var_name)),
            LangTypeEnum::Array { value, .. } => {
                if let LangTypeEnum::Int { value: Int { value: length }, .. } = *value {
                    let element_size = match *value {
                        LangTypeEnum::Int { .. } => 8,
                        LangTypeEnum::Float { .. } => 8,
                        LangTypeEnum::Char { .. } => 1,
                        _ => 8,
                    };
                    nasm_code.push_str(&format!("    {} resq {}\n", var_name, length * element_size));
                } else {
                    nasm_code.push_str(&format!("    {} resq 1\n", var_name));
                }
            }
            _ => nasm_code.push_str(&format!("    {} resq 1\n", var_name)),
        }
    }

    nasm_code
}