//
//bool:print:string*

pub fn nasm_print_function() -> &'static str {
    r#"
%ifdef LINUX
print:
    pop rsi
    push rsi
    call strlen         ; rax = 文字数
    pop rdx             ; rsi = 文字列アドレス（write用）

    mov rax, 1          ; sys_write
    mov rdi, 1          ; stdout
    ; rsi = 文字列アドレス（既に設定済み）
    syscall
    push rax
    ret
%endif

%ifdef WINDOWS
print:
    ; Windows用print関数の実装はここに書く
    ret
%endif
strlen:
    pop rdi              ; rdi = 文字列アドレス
    xor rcx, rcx         ; rcx = カウンタ = 0
.next:
    mov al, byte [rdi]   ; 1バイト読み込み
    cmp al, 0            ; null 文字？
    je .done
    inc rdi              ; 次の文字へ
    inc rcx              ; カウント
    jmp .next
.done:
    push rcx    
    ret
"#
}
