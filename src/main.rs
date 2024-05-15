use std::arch::asm;

fn printf(buf: &str) {
    unsafe {
        asm!(
            "mov rax, 1",
            "mov rdi, 1",
            "syscall",

            "mov rax, 60",
            "mov rdi, 0",
            "syscall",

            in("rsi") buf.as_ptr(),
            in("rdx") buf.len(),
        );
    }
}

fn main() {
    printf("Hello, world!\n");
}
