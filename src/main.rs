pub mod instruction;

fn main() {
    let a:u16 = 0xffff;
    let b:i8 = -127;
    let c:i32 = (a as i32) + ( b as i32);
    println!("Result {}", c as u16);
}
