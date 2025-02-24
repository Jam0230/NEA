global main
extern printf
extern puts
section .data

section .text
main:
push rbp
mov rbp,rsp
sub rsp,4
mov eax, 0
mov dword[rbp-4], eax
.0:
mov eax, dword[rbp-4]
mov ecx, 3
cmp rax, rcx
setge al
cmp al,1
jne .1
mov eax, dword[rbp-4]
mov ecx, 1
add rax,rcx
mov dword[rbp-4], eax
jmp .0
.1:
add rsp,8
pop rbp
ret