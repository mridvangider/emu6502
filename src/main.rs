pub mod instruction;
pub mod cpu;
pub mod util;
pub mod errors;

fn main() {
    let a:u16 = 0xffff;
    let b:i8 = -127;
    let c:i32 = (a as i32) + ( b as i32);
    println!("Result {}", c as u16);
}
