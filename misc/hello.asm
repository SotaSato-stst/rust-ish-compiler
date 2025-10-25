global _main
default rel

; 出力したいデータを格納する領域
section .data
    ; 1バイトの数字と改行コードを格納するバッファ
    ; 初期値はダミーで、実行時に書き換えられます。
    output_buffer db " ", 0x0a

section .text
_main:
    mov rdi, 1
    mov rsi, 2
    call sum
    add al, '0'

    lea rdi, output_buffer
    mov [rdi], al

    mov rax, 0x2000004
    mov rdi, 1
    lea rsi, output_buffer
    mov rdx, 2
    syscall

    ; 5. exit システムコール
    mov rax, 0x2000001
    xor rdi, rdi
    syscall

sum:
    mov rax, rdi
    add rax, rsi
    ret