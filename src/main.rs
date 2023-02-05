use std::arch::asm;

fn main() {
    const MESSAGE: [u8; 14] = [0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21, 0x0a];
    write_syscall(MESSAGE);
}

#[cfg(not(target_os = "windows"))]
#[inline(never)]
fn write_syscall(message: [u8; 14]) {
    let msg_ptr = message.as_ptr();
    let len = message.len();
    unsafe {
        if cfg!(target_os = "macos") {
            asm!("mov rax, 0x2000004");
        } else if cfg!(target_os = "linux") {
            asm!("mov rax, 1");
        }
    }

    unsafe {
        asm!(
            "mov rdi, 1",
            "syscall",
            in("rsi") msg_ptr,
            in("rdx") len,
            out("rax") _, out("rdi") _, lateout("rsi") _, lateout("rdx") _
        );
    }
}