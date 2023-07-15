use std::io;

fn main() {
    let mut choice = String::new();

    loop {
        println!("Please select an operation:");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Quit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        choice = input.trim().to_string();

        match choice.as_str() {
            "1" => add(),
            "2" => subtract(),
            "3" => multiply(),
            "4" => divide(),
            "5" => break,
            _ => println!("Invalid choice"),
        }
    }
}

fn add() {
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Enter two numbers:");
    io::stdin().read_line(&mut num1).unwrap();
    io::stdin().read_line(&mut num2).unwrap();

    let x = num1.trim().parse::<i32>().unwrap();
    let y = num2.trim().parse::<i32>().unwrap();

    println!("The sum is {}", x + y);
}

fn subtract() {
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Enter two numbers:");
    io::stdin().read_line(&mut num1).unwrap();
    io::stdin().read_line(&mut num2).unwrap();

    let x = num1.trim().parse::<i32>().unwrap();
    let y = num2.trim().parse::<i32>().unwrap();

    println!("The difference is {}", x - y);
}

fn multiply() {
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Enter two numbers:");
    io::stdin().read_line(&mut num1).unwrap();
    io::stdin().read_line(&mut num2).unwrap();

    let x = num1.trim().parse::<i32>().unwrap();
    let y = num2.trim().parse::<i32>().unwrap();

    println!("The product is {}", x * y);
}

fn divide() {
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Enter two numbers:");
    io::stdin().read_line(&mut num1).unwrap();
    io::stdin().read_line(&mut num2).unwrap();

    let x = num1.trim().parse::<i32>().unwrap();
    let y = num2.trim().parse::<i32>().unwrap();

    println!("The quotient is {}", x / y);
}
