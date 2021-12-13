fn main() {
    let x: i8 = 127;
    let y: i8 = 28;
    let z = 3.14;
    let a = true;
    let all = (5, "hello", false);
    
    let sum = 5 + 5;
    let diff = 6 - 2;
    let prod = 8 * 5;
    let quot = 9 / 8;
    let rem = 56 % 5;
    let array = [1, 2, 3, 4, 5];

    println!("x = {}", x);
    println!("y = {}",y);
    println!("z = {}", z);
    println!("a = {}", a);
    println!("sum = {}", sum);
    println!("diff = {}", diff);
    println!("prod = {}", prod);
    println!("quot = {}", quot);
    println!("rem = {}", rem);
    println!("array = {:?}", array);
    println!("array[0] = {}", array[0]);
    println!("array[1] = {}", array[1]);
    println!("hello = {}", all.1);
    
}
