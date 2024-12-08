;------------
; loop.s
;------------
bits 64
section .text

extern printf

global _start

sum: ; int sum(int a, int b)
  MOV rax, rdi       ; Use rdi value (a)
  ADD rax, rsi       ; Add rsi value (b)
  RET

total: ; int total(int a)
  MOV rcx, 0      ; counter
  MOV rax, 0      ; total

loop:
  INC rcx            ; counter++
  ADD rax, rcx       ; total += counter
  CMP rcx, rdi       ; compare counter with the limit (a)
  JNE loop           ; jump if counter != limit
  RET

_start:
  ; Example usage of total
  MOV rdi, 10        ; Calculate the sum of numbers 1 to 10
  CALL total         ; Call the total function
                     ; Result will be in RAX

  MOV rdi, message  ; printf(message,total)
  MOV rsi, rax
  CALL printf

  ; Exit the program
  MOV rdi, 0         ; Exit code 0
  MOV rax, 60        ; syscall: exit
  SYSCALL

section .data
    message db "Result: %d", 10, 0   ; Format string with newline
