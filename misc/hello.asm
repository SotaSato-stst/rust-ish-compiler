; macOS x86_64用 "Hello, World!" プログラム (NASM形式)

global _main      ; プログラムのエントリポイントを定義
default rel       ; RIP相対アドレッシングをデフォルトにする (モダンなmacOSで推奨)

section .data     ; データセクション
    msg db "Hello, World!", 0x0A ; 出力する文字列と改行コード(0x0A)
    msg_len equ $ - msg         ; 文字列の長さ

section .text     ; コードセクション
_main:            ; main関数に相当するエントリポイント

; 1. writeシステムコール (システムコール番号: 0x2000004)
    mov rax, 0x2000004  ; raxにwriteのシステムコール番号を設定
    mov rdi, 1          ; rdiに第一引数: ファイルディスクリプタ (1: 標準出力/stdout)
    lea rsi, [msg]      ; rsiに第二引数: 出力する文字列のアドレス（RIP相対）
    mov rdx, msg_len    ; rdxに第三引数: 出力するバイト数
    syscall             ; システムコールを実行

; 2. exitシステムコール (システムコール番号: 0x2000001)
    mov rax, 0x2000001  ; raxにexitのシステムコール番号を設定
    xor rdi, rdi        ; rdiに第一引数: 終了コード (0)
    syscall             ; システムコールを実行