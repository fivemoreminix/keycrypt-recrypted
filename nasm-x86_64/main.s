global main
extern puts
extern strlen

sys_write   equ     1   ; Linux WRITE syscall number
sys_exit    equ     60  ; Linux EXIT syscall number

; Linux file descriptors
stdin       equ     0   ; standard input
stdout      equ     1   ; standard output
stderr      equ     2   ; standard error

%macro write 3
    mov rax, sys_write
    mov rdi, %1         ; file descriptor
    mov rsi, %2         ; buffer
    mov rdx, %3         ; buffer length in bytes
    syscall
%endmacro

            section .data

help        db `USAGE: keycrypt words=FILENAME \"MESSAGE\"`, 10
help_len    equ $ - help

filename_len dq 0

            section .bss

filename    resb    512     ; reserve 512 uninitialized bytes for the words list filename

            section .text

main:
            pop     r8                              ; pop number of arguments from the stack
            pop     rsi                             ; discard the program name

            test    r8, 3                           ; make sure we have exactly 3 arguments
            je      .argloop                        ; if so, begin the argument parsing

            write   stdout, help, help_len          ; not a valid number of arguments; print usage
            xor     rax, sys_exit
            mov     rdi, $1
            syscall                                 ; return error
            ret

.argloop:   cmp     r8, 0                           ; check if we have more arguments to iterate
            jz      .exit

            mov     rax, sys_write
            mov     rdi, stdout
            pop     rsi
            mov     rdx, $5                         ; TODO: use C strlen function
            syscall

            dec     r8                              ; count down
            jmp     .argloop


.exit:      mov     rax, sys_exit
            xor     rdi, rdi
            syscall

            ret
