pub fn generate_windows_code(ir: &str) -> String {
    format!("Windows machine code for: {}", ir)
}

/// Translate SIL (Structured Intermediate Language) to NASM assembly code.
pub fn translate_to_nasm(sil: &str) -> String {
    let mut nasm_code = String::new();

    // 行ごとにSILコードを処理
    for line in sil.lines() {
        let trimmed = line.trim();

        // PUSH命令: 定数をスタックに積む
        if trimmed.starts_with("PUSH") {
            let value = trimmed.split_whitespace().nth(1).unwrap_or("0");
            nasm_code.push_str(&format!("    push {}\n", value));

        // ADD命令: スタックトップ2つを加算
        } else if trimmed.starts_with("ADD") {
            nasm_code.push_str("    pop rax\n    pop rbx\n    add rax, rbx\n    push rax\n");

        // SUB命令: スタックトップ2つを減算
        } else if trimmed.starts_with("SUB") {
            nasm_code.push_str("    pop rbx\n    pop rax\n    sub rax, rbx\n    push rax\n");

        // MUL命令: スタックトップ2つを乗算
        } else if trimmed.starts_with("MUL") {
            nasm_code.push_str("    pop rax\n    pop rbx\n    imul rax, rbx\n    push rax\n");

        // DIV命令: スタックトップ2つを除算
        } else if trimmed.starts_with("DIV") {
            nasm_code.push_str("    pop rbx\n    pop rax\n    cqo\n    idiv rbx\n    push rax\n");

        // STORE命令: スタックトップを変数に保存
        } else if trimmed.starts_with("STORE") {
            let var = trimmed.split_whitespace().nth(1).unwrap_or("unknown");
            nasm_code.push_str(&format!("    pop [{}]\n", var));

        // LOAD命令: 変数の値をスタックに積む
        } else if trimmed.starts_with("LOAD") {
            let var = trimmed.split_whitespace().nth(1).unwrap_or("unknown");
            nasm_code.push_str(&format!("    push [{}]\n", var));

        // JMP命令: 無条件ジャンプ
        } else if trimmed.starts_with("JMP") {
            let label = trimmed.split_whitespace().nth(1).unwrap_or("unknown");
            nasm_code.push_str(&format!("    jmp {}\n", label));

        // JMPIF命令: 条件付きジャンプ
        } else if trimmed.starts_with("JMPIF") {
            let label = trimmed.split_whitespace().nth(1).unwrap_or("unknown");
            nasm_code.push_str("    pop rax\n    cmp rax, 1\n");
            nasm_code.push_str(&format!("    je {}\n", label));

        // ラベル定義
        } else if trimmed.starts_with("LABEL") || trimmed.ends_with(":") {
            nasm_code.push_str(&format!("{}:\n", trimmed.trim_end_matches(':')));

        // 未対応の命令
        } else {
            nasm_code.push_str(&format!("    ;; Unhandled instruction: {}\n", trimmed));
        }
    }

    nasm_code
}
