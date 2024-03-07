;------------
; sum.s
;------------
bits 64
section .text
global sum, total


sum: ;int sum(int a, int b)
  MOV rax, rdi       ; Use rdi value
  ADD rax, rsi       ; Add rsi value
  RET

total: ;int total(int a)
  MOV rcx, 0      ; counter
  MOV rax, 0      ; total
loop:
  INC rcx
  ADD rax, rcx
  CMP rcx, rdi ; compare counter and limit
  JNE loop
  RET

