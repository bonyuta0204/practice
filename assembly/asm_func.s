;------------
; sum.s
;------------
bits 64
section .text
global sum


sum: ;int sum(int a, int b)
  MOV rax, rdi       ; Use rdi value
  ADD rax, rsi       ; Add rsi value
  RET
