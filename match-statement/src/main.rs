fn main() {
    let x = 10;

    match x {
        1 => println!(" Value of x is 1"),
        2 => println!(" value of x is 2"),
        _=> println!(" value of x is iunvalid"),

    }

    let x = false;
    let y = true;

    match (x,y ){
        (true,true) => println!(" x and y are true"),
        (true,false) => println!(" x is true and y is false"),
        (false,true) => println!(" x is false and y is true"),
        (false,false) => println!(" x and y are false"),
    }
}
