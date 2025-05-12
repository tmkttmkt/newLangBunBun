pub fn generate_linux_code(ir: &str) -> String {
    format!("Linux machine code for: {}", ir)
}

/// Translate SIL (Structured Intermediate Language) to Linux NASM assembly code.
pub fn translate_to_linux_nasm(sil: &str) -> String {
    let mut nasm_code = String::new();
    

    nasm_code.push_str("section .text\n");
    nasm_code.push_str("_start:\n");

    for line in sil.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("PUSH") {
            let value = trimmed.split_whitespace().nth(1).unwrap_or("0");
            nasm_code.push_str(&format!("    push {}\n", value));
        } else if trimmed.starts_with("ADD") {
            nasm_code.push_str("    pop rax\n    pop rbx\n    add rax, rbx\n    push rax\n");
        } else if trimmed.starts_with("SUB") {
            nasm_code.push_str("    pop rbx\n    pop rax\n    sub rax, rbx\n    push rax\n");
        } else if trimmed.starts_with("MUL") {
            nasm_code.push_str("    pop rax\n    pop rbx\n    imul rax, rbx\n    push rax\n");
        } else if trimmed.starts_with("DIV") {
            nasm_code.push_str("    pop rbx\n    pop rax\n    cqo\n    idiv rbx\n    push rax\n");
        } else if trimmed.starts_with("STORE") {
            let var = trimmed.split_whitespace().nth(1).unwrap_or("unknown");
            nasm_code.push_str(&format!("    pop [{}]\n", var));
        } else if trimmed.starts_with("LOAD") {
            let var = trimmed.split_whitespace().nth(1).unwrap_or("unknown");
            nasm_code.push_str(&format!("    push [{}]\n", var));
        } else if trimmed.starts_with("JMP") {
            let label = trimmed.split_whitespace().nth(1).unwrap_or("unknown");
            nasm_code.push_str(&format!("    jmp {}\n", label));
        } else if trimmed.starts_with("JMPIF") {
            let label = trimmed.split_whitespace().nth(1).unwrap_or("unknown");
            nasm_code.push_str("    pop rax\n    cmp rax, 1\n");
            nasm_code.push_str(&format!("    je {}\n", label));
        } else if trimmed.starts_with("LABEL") || trimmed.ends_with(":") {
            nasm_code.push_str(&format!("{}:\n", trimmed.trim_end_matches(':')));
        } else {
            nasm_code.push_str(&format!("    ;; Unhandled instruction: {}\n", trimmed));
        }
    }

    // プログラム終了コードを追加
    nasm_code.push_str("\n    mov rax, 60         ; syscall: exit\n");
    nasm_code.push_str("    xor rdi, rdi        ; status = 0\n");
    nasm_code.push_str("    syscall\n");
    nasm_code.push_str("section .data\n");

    // 変数の定義を追加
    // nasm_code.push_str("result dq 0\n");
    // nasm_code.push_str("newline db 10\n");
    // nasm_code.push_str("buffer db 0, 0, 0, 0, 0, 0, 0, 0, 10 ; 出力用（改行含む）\n\n");

    nasm_code
}