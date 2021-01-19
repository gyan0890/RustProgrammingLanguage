use std::io;

fn main() {
    /*
    Create a mutable variable called input to get the choice from user
    A: Fahrenheit to Celsius
    B: Celsium to Fahrenheit
    */
    let mut input = String::new();
    println!("Enter A for Fahrenheit to Celsius");
    println!("Enter B for Celsius to Fahrenheit");
    
    //Syntax to get the input from the console
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    let input: char = input.trim().parse().expect("invalid input");
    
    //Temperature conversion logic based on the input entered by the user
    if input == 'A' {
        println!("Enter the temperature in Fahrenheit");
        let mut F = String::new();
        io::stdin()
            .read_line(&mut F)
            .expect("failed to read from stdin");
        
        let F: i32 = F.trim().parse().expect("invalid input");
        let mut C = 0;
        C = (F - 32)*5/9;
        
        println!("The temperature in Celsius is: {}", C);
    }
    else if input == 'B' {
     println!("Enter the temperature in Celsius");
        let mut C = String::new();
        io::stdin()
            .read_line(&mut C)
            .expect("failed to read from stdin");
        
        let C: i32 = C.trim().parse().expect("invalid input");
        let mut F = 0;
        F = (C*9/5) + 32;
        println!("Temperate in Fahrenheit is: {}", F);
    }
    else {
        println!("You have chosen a wrong option!");
    }
    
    /* Comments
    Can enhance using loops
    Keep taking user inputs and performing the operations till user wants to stop
    */
    
}
