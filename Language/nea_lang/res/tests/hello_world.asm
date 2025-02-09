global main
extern printf
extern puts
section .data
f0: dq 0.1
f1: dq 0.3
section .text
main:
push rbp
mov rbp,rsp
sub rsp,8
movsd xmm0, [f0]
movsd xmm1, [f1]
movsd xmm2, xmm0
divsd xmm2, xmm1
push rax
cvttsd2si rax, xmm2
cvtsi2sd xmm2, rax
pop rax
mulsd xmm2, xmm1
subsd xmm0, xmm2
movsd qword[rbp-8], xmm0
add rsp,8
pop rbp
ret