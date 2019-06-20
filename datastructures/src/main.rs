#![allow(dead_code)]
#![allow(unused_variables)]
mod pattern_matching;

fn main() {
    //structures();
    //enums();
    //option();
    //array();
    //vectors();
    //slices();
    //strings();
    //tuples();
    //pattern_matching::pattern_matching();
    //generics();
    traits();
}

struct Line {
    start: Point,
    end: Point
}
struct Point {
    x: f64,
    y: f64
}

fn structures() {
    let p = Point {x: 2.2, y: 4.0};
    println!("Point p is at ({}, {})", p.x, p.y);

    let p2 = Point {x: 5.0, y: 10.0};

    let myline = Line {start: p, end: p2};

}


enum Color {
    Red, 
    Green, 
    Blue,
    RgbColor(u8, u8, u8), //tuple
    CmykColor{cyan: u8, magenta: u8, yellow: u8, black: u8}, //Struct
}
fn enums() {
    let c:Color = Color::CmykColor{cyan: 0, magenta: 128, yellow: 0, black: 255};

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0)
        | Color::CmykColor{cyan:_, magenta:_,yellow:_, black:255}=> println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
        _=> ()
    }
}

fn option() {
    //Option<T>
    let x = 3.0;
    let y = 2.0;

    //some(z) or None
    let result:Option<f64> = if y != 0.0 { Some(x/y) } else { None };
    println!("{:?}", result);

    match result {
        Some(z) => println!("{}/{} is {}", x, y, z),
        None => println!("Cannot divide {} by {}", x, y)
    }

    //if let and while let
    if let Some(z) = result {
        println!("z = {}", z);
    }
}

fn array() {
    let mut a: [i32; 5] = [0, 1, 2, 3, 4];

    println!("a has {} elements, first is {}", a.len(), a[0]);
    a[0] = 69;
    println!("a[0] = {}", a[0]);

    println!("{:?}", a);
}

fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("{:?}", a);

    a.push(69);

    //usize, isize
    let idx: usize = 0;

    a[idx] = 169;

    println!("{:?}", a);

    //Option
    match a.get(3) {
        Some(x) => println!("a[6] = {}", x),
        None => println!("Error, no element")
    }

    for x in &a {
        println!("{}", x);
    }

    a.push(49);
    println!("{:?}", a);

    //Some(49)
    let last_element = a.pop(); //Option 
    println!("The last element is {:?} and the full vector is {:?}", last_element, a);

    while let Some(x) = a.pop() {
        println!("x= {}", x);
    }
}

fn use_slice(slice: &mut[i32]) {
    println!("First elem = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;
}

fn slices() {
    let mut data = [1, 2, 3, 4, 5];

    //use_slice(&mut data[1..4]);
    use_slice(&mut data);

    println!("{:?}", data);
}

fn strings() {
    //utf-8 characters
    let s:&'static str = "Rustyyy!"; //&str = string slice

    //s = "abd";
    //let h = s[0];

    for c in s.chars().rev() {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("The first letter is {}", first_char);
    }

    // A string is a heap allocated construct
    let mut letters = String::new();
    let mut a = 'a' as u8;

    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("{}", letters);

    //&str <> String
    let u: &str = &letters; //Type coercion

    //Concatenation
    // String + str
    //let z = letters + &letters;

    //let mut abc = String::from("hello");
    let mut abd = "hello world".to_string();
    println!("{}", abd);

    abd.remove(0);
    abd.push_str("!!"); 
    println!("{}", abd.replace("ello", "goodbye"));    

}

fn tuples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x,y);

    println!("{:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    //Destructuring
    let (a,b) = sp;
    println!("a = {}, b = {}", a, b);

    let sp2 = sum_and_product(6,7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("last elem = {}", (combined.1).1);

    let ((c,d),(e,f)) = combined;

    let foo = (true, 69, -1i8);
    println!("{:?}", foo);

    let meaning = (69,);
    println!("{:?}", meaning);
}

fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x+y, x*y)
}

struct Pointy<T> {
    x: T,
    y: T
}

struct Liney<T> {
    start: Pointy<T>,
    end: Pointy<T> 
}

fn generics() {
    let a = Pointy { x: 0f64, y: 4f64 };
    let b = Pointy { x: 1.2, y: 3.4};

    let my_liney = Liney { start: a, end: b };
}

//Traits
fn traits() {
    //let h = Human{name:"Dickson"};
    //let h = Human::create("Dickson");
    let h:Human = Animal::create("Dickson");
    h.talk();

    //let c = Cat{name:"Pussie"}; 
    let c = Cat::create("Pussie");
    c.talk();

    //Trait summable
    let a = vec![1, 2, 3];
    println!("sum = {}", a.sum());
}

trait Animal {
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("The {} cannot talk", self.name());
    }
}

trait Summable<T> {
    fn sum(&self) -> T;
}
impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result:i32 = 0;
        for x in self {
            result += *x;
        }
        result
    }
}

struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str,

}

impl Animal for Human {
    fn create(name: &'static str) -> Human {
        Human{name: name}
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says hello", self.name());
    }
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        Cat{name: name}
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says meoaww", self.name());
    }
}
 


