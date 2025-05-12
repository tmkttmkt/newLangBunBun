SECTION .data
x dd 5

SECTION .text
global _start

_start:

    ; x > 0
    mov eax, [x]
    cmp eax, 0
    jg then_branch

    ; else branch
    mov eax, [x]
    sub eax, 5
    mov [x], eax
    jmp end_program

then_branch:
    mov eax, [x]
    sub eax, 1
    mov [x], eax

end_program:
    ; exit(0)
    mov eax, 1
    xor ebx, ebx
    int 0x80
