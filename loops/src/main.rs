fn main() {

    //loops
    
    let mut x = 1;
    loop{
        x = x*2;
        if x>=512 {
            break;

        }
        println!("The value of x new is {}",x);
    }
    let mut y = 1;
    while y<500{
        y =y*2;
        println!("The value of y new is {}",y);
    }

    for z in 0..9{
        println!("The value of z is {}",z);
    }
    println! ("outside the loop");

    
    for zi in 0..=9{
        println!("The value of zi is {}",zi);
    }
}
