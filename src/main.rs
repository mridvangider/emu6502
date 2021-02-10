pub mod instruction;

fn le2be(val: &u16) -> u16 {
    let mut ret: u16;
    ret = val & 0x00FF;
    ret = ret << 8;
    ret |= val >> 8;
    return ret;
}

fn main() {
    let a:u16 = 0xffff;
    let b:i8 = -127;
    let c:i32 = (a as i32) + ( b as i32);

    println!("Result {}", c);
}
