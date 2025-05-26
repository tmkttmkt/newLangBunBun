section .text

print:
    ; print関数の実装はここに書く
    ret
_start:
main:
    push 2
    push 3
    pop rax
    pop rbx
    add rax, rbx
    push rax
    pop [result]
    call print_result
    pop [return]
print_result:
    push [result]
    call syscall_write
    pop [return]

    mov rax, 60         ; syscall: exit
    xor rdi, rdi        ; status = 0
    syscall

section .bss
