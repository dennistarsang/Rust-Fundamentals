#![allow(dead_code)]
#![allow(unused_variables)]

fn if_statement() {
    let temp = 32;

    if temp > 30 {
        println!(" Really hot outside"); 
    } 
    else if temp < 10 {
        println!("Really cold outside");
    }
    else {
        println!("Temp is OK");
    }

    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("The day is {}", day);

    println!("is it {}", if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"okay"});

    println!("It is {}", 
        if temp > 20 {
            if temp > 30 {"very hot"} else {"hot"}
            }
        else if temp < 10 {"cold"} else {"OK"});
}

fn while_loop() {
    let mut x = 1;

    while x < 1000 {
        x *= 2;

        if x == 64 {
            continue;
        }

        println!("x is = {}", x);
    }

    //Loop ...while true
    let mut y = 1;

    loop { //while true
        y *= 2;
        println!("y = {}", y);

        if y == 1<<10 {
            break;
        }
    }
}

fn for_loop() {
    for x in 1..11 {
        if x == 3 {
            continue;
        }
        if x == 9 {
            break;
        }

        println!("x = {}", x);
    }

    for (pos,y) in (30..41).enumerate() { //exclusive of 41
        println!("{}: {}", pos, y);
    }
}

fn match_statement() {
    let country_code = 999;

    let country = match country_code {
        44 => "UK",
        33 => "France",
        254 => "Kenya",
        1...999 => "Unknown", //Inclusive of 999
        _ => "Invalid"
    };

    println!("The country with code {} is {}", country_code, country);
}

fn main() {
    //if_statement();
    //while_loop();
    //for_loop();
    match_statement();
}
