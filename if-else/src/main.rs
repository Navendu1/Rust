fn main() {
    
    let num = 5;

    if num == 5 {
        println!("You Won!");
    } else if num == 6 {
        println!("Try Again!");
    } else {
        println!("You Lost!");
    }
    divisible();
}

    fn divisible() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}



