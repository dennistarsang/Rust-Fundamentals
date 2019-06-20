use std::mem;

fn main() {
    scope_and_shadowing();
    fundamental_data_types();
}

fn scope_and_shadowing() {
    let a = 127;
    
    println!("a = {}", a);
    {
        let b = 457;
        println!("inside, b = {}", b);
        let a = 133;
        println!("inside a = {}", a);
        
    }
    println!("a = {}", a);
}
fn fundamental_data_types() {
    let a:u8 = 123; //8bits
    println!("a = {}", a);

    //mut
    let mut b:i8 = 55; //mutable
    println!("B is {}", b);

    b = 20;

    println!("B is {}", b);

    let mut c = 123456789; //32 bit signed i32
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("C is {} after modification", c);


     //i8, u8, i16, u16, i32, u32, i64, u64
    let z:isize = 123; //isize .. size of memory address
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);

    //Char
    let d:char = 'X';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    //Float
    let e:f64 = 2.5; //Double precision, 8 bytes
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    //Bool
    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));

}

