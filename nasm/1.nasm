main:
    push [x]
    push 0
    ;; Unhandled instruction: GT
    ;; Unhandled instruction: DUP
    jmp :label_0
    push -1
    pop rax
    pop rbx
    imul rax, rbx
    push rax
    jmp :label_1
    ;; Unhandled instruction: 
label_0:
    push [x]
    push 1
    pop rbx
    pop rax
    sub rax, rbx
    push rax
    pop [x]
    pop [return]
    ;; Unhandled instruction: 
label_1:
    push [x]
    push 5
    pop rbx
    pop rax
    sub rax, rbx
    push rax
    pop [x]
    pop [return]
