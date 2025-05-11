section .data
result      dq 0
newline     db 10
buffer      db 0, 0, 0, 0, 0, 0, 0, 0, 10 ; 出力用（改行含む）

section .text
global _start

_start:
    ; --- main ---
    mov rax, 2
    push rax
    mov rax, 3
    push rax
    pop rbx
    pop rax
    add rax, rbx
    mov [result], rax
    call print_result

    ; --- exit ---
    mov rax, 60         ; syscall: exit
    xor rdi, rdi        ; status = 0
    syscall

print_result:
    ; 数値を文字列に変換（itoa風、1桁限定）
    mov rax, [result]
    add rax, '0'
    mov [buffer], al

    ; write(1, buffer, 2)
    mov rax, 1          ; syscall: write
    mov rdi, 1          ; stdout
    lea rsi, [buffer]
    mov rdx, 2          ; 1文字+改行
    syscall
    ret
