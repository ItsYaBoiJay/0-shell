use std::process;

pub fn exit(code: i32) -> ! {
    process::exit(code);
}
