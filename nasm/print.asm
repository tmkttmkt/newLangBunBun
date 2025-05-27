




%ifdef LINUX
print:
    pop r15             ;戻りアドレス
    pop rsi
    push r15            ;戻りアドレスを戻す
    push rbp            ;退避
    mov rbp, rsp        ;関数スタック

    push rsi
    call strlen         ; rax = 文字数
    pop rdx             ; rsi = 文字列アドレス（write用）

    mov rax, 1          ; sys_write
    mov rdi, 1          ; stdout
    ; rsi = 文字列アドレス（既に設定済み）
    syscall
    
    mov rsp, rbp        ;関数スタックを破棄
    pop rbp             ;退避を戻す
    pop r15             ;戻りアドレスを一時退避
    push rax            ;返り値
    push r15            ;戻りアドレス
    ret
%endif

%ifdef WINDOWS
print:
    ; Windows用print関数の実装はここに書く
    ret
%endif
strlen:
    pop r15             ;戻りアドレス
    pop rdi             ;引数 
    push r15            ;戻りアドレスを戻す
    push rbp            ;退避
    mov rbp, rsp        ;関数スタック

    xor rcx, rcx         ; rcx = カウンタ = 0
.next:
    mov al, byte [rdi]   ; 1バイト読み込み
    cmp al, 0            ; null 文字？
    je .done
    inc rdi              ; 次の文字へ
    inc rcx              ; カウント
    jmp .next
.done:

    mov rsp, rbp        ;関数スタックを破棄
    pop rbp             ;退避を戻す
    pop r15             ;戻りアドレスを一時退避
    push rcx            ;返り値
    push r15            ;戻りアドレス
    ret

