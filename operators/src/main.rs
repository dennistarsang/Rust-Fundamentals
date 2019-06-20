use std::mem;

const MEANING_OF_LIFE: u8 = 69; // No fixed address

static mut Z: i32 = 169; // Has a fixed address

fn operators() {
    //Arithmtic
    let mut a = 2+3*4;
    println!("{}", a);
    a += 1;
    println!("a is {} now", a);
    a -= 2;

    println!("Remainder of {} / {} is {}", a, 3, (a%3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is = {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    //Bitwise
    let c = 1 | 2; // | OR & AND ^ XOR ! NOR
    println!("1 | 2 = {}", c);

    let two_to_ten = 1 << 10;
    println!("2^10 = {}", two_to_ten);

    //Logical
    let pi_less_4 = std::f64::consts::PI < 4.0;
    println!("pi is less than 4 = {}", pi_less_4);
}

fn main() {
    unsafe {
        Z = 269;
        println!("{}", Z);
    }
}
