#![allow(dead_code)]
#![allow(unused_variables)]


fn is_even(x: u32) -> bool {
    x % 2 == 0
}

fn main() {
    //functions();
    //methods();
    //closures();

    //higher order functions
    let limit = 500;
    let mut sum = 0;

    for i in 0.. {
        let isq = i * i;

        if isq > limit {
            break;
        } else if is_even(isq) {
            sum += isq;
        }
    }
    println!("Loop sum is {}", sum);

    let sum2 = 
        (0..).map(|x| x*x)
            .take_while(|&x| x < limit)
            .filter(|x| is_even(*x))
            .fold(0, |sum,x| sum+x);
    println!("hof sum = {}", sum2);
}

fn functions() {
    print_value(69);

    let mut z = 1;
    increase(& mut z);
    println!("z = {}", z);

    let a = 2;
    let b = 3;
    let p = product(a,b);
    println!("The product of {} and {} is {}", a, b, p);

}

fn increase(x: &mut i32) {
    *x += 1;
}

fn print_value(x: i32) {
    println!("value = {}", x);
}

fn product(x: i32, y: i32) -> i32 {
    x * y
}

//methods
struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

impl Line {
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx*dx+dy*dy).sqrt()
    }
}

fn methods() {
    let p = Point { x: 2.0, y: 3.0 };
    let p2 = Point { x: 4.0, y: 5.0 };
    let myline = Line { start: p, end: p2 };

    println!("length = {}", myline.len());
}

//closures
fn say_hello() {
    println!("hello");
}

fn closures() {
    let sh = say_hello;
    sh();

    let plus_one = |x: i32| -> i32 { //Pass by value
        x + 1
    };
    let a = 68;
    println!("{} + 1 = {}", a, plus_one(a));

    let mut two = 2;
    let plus_two = |x| {
        let mut z = x;
        z += two;
        z
    };
    println!("{} + 2 = {}", 67, plus_two(67));

    let borrow_two = &mut two;

    //T : Pass by value
    //&T : Pass by reference
    //&mut T : Mutable reference
    let plus_three = |x: &mut i32| *x += 3;
    let mut f = 12;
    plus_three(&mut f);
    println!("f is {}", f);

}