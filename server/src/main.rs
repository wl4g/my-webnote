use mywebnote::cmd;

// see:https://rustcc.cn/article?id=75f290cd-e8e9-4786-96dc-9a44e398c7f5
// Check for the allocator used: 'objdump -t target/debug/mywebnote | grep mi_os_alloc'
#[global_allocator]
//static GLOBAL: std::alloc::System = std::alloc::System;
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub fn main() {
    cmd::execute_commands_app();
}
