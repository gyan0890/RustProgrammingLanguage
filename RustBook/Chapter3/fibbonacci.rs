use std::io;

fn main() {
    let mut num1 = 1;
    let mut num2 = 1;

    println!("Enter the value of n ");
    
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("failed to read from stdin");
        
    let mut n: i32 = n.trim().parse().expect("invalid input");

    while n-2 > 0 {
        let num3 = num1 + num2;
        num1 = num2;
        num2 = num3;
        n = n-1;
    }

    println!("n-th fibbonacci number is {}", num2);
    



}