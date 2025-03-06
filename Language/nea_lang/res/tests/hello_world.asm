global main
extern printf
extern puts
section .data
s0: db "FizzBuzz",10, 0
s1: db "Fizz",10, 0
s2: db "Buzz",10, 0
s3: db "%i",10, 0
section .text
main:
push rbp
mov rbp,rsp
sub rsp,4
mov eax, 1
mov dword[rbp-4], eax
.0:
mov eax, dword[rbp-4]
mov ecx, 100
cmp rax, rcx
setle al
cmp al,1
jne .1
mov eax, dword[rbp-4]
mov ecx, 3
mov rax, rax
xor rdx, rdx
div rcx
mov rax, rdx
mov ecx, 0
cmp rax, rcx
sete al
mov ecx, dword[rbp-4]
mov edx, 5
push rax
mov rax, rcx
mov r11, rdx
xor rdx, rdx
div r11
mov rcx, rdx
pop rax
mov edx, 0
cmp rcx, rdx
sete cl
and al, cl
cmp al, 1
jne .2
push rax
xor rax, rax
mov rdi, s0
call printf
pop rax
jmp .5
.2:
mov eax, dword[rbp-4]
mov ecx, 3
mov rax, rax
xor rdx, rdx
div rcx
mov rax, rdx
mov ecx, 0
cmp rax, rcx
sete al
cmp al, 1
jne .3
push rax
xor rax, rax
mov rdi, s1
call printf
pop rax
jmp .5
.3:
mov eax, dword[rbp-4]
mov ecx, 5
mov rax, rax
xor rdx, rdx
div rcx
mov rax, rdx
mov ecx, 0
cmp rax, rcx
sete al
cmp al, 1
jne .4
push rax
xor rax, rax
mov rdi, s2
call printf
pop rax
jmp .5
.4:
push rax
xor rax, rax
mov rdi, s3
mov esi, dword[rbp-4]
call printf
pop rax
.5:
mov eax, dword[rbp-4]
mov ecx, 1
add rax,rcx
mov dword[rbp-4], eax
jmp .0
.1:
add rsp,4
pop rbp
ret