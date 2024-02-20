;------------
; hello.s
;------------

bits 64
section .text
global _start


_start:
  xor    eax, eax
  mov    edx, eax
  inc    eax
  mov    edi, eax
  mov    dl, len
  mov    rsi, msg
  syscall
  xor   edi,edi
  mov   eax,edi
  mov   al,60
  syscall

section .data
  msg    db   "hello world"
  len    equ  $ - msg
