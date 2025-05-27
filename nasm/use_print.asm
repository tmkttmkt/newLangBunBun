section .data
    msg db "Hello, world!",10, 0

section .text
    global _start

_start:
    lea rsi, [rel msg]   ; 文字列アドレスをrsiにセット
    push rsi             ; print関数用にアドレスをpush
    call print           ; print関数呼び出し
    pop rax
    ; 終了処理
    mov rax, 60          ; syscall: exit
    xor rdi, rdi         ; status = 0
    syscall

%include "print.asm"
