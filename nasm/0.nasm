section .text

print:
    ; print関数の実装はここに書く
    ret
_start:
    push 2
    push 3
    pop rax
    pop rbx
    add rax, rbx
    push rax
    pop [result]

    mov rax, 60         ; syscall: exit
    xor rdi, rdi        ; status = 0
    syscall

section .bss
    result resq 1
