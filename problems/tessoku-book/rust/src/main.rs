fn variables() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn shadowing() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 1;
    println!("The value of x is: {}", x)
}

fn error_pattern() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = x + 1;
    println!("The value of x is: {}", x);
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x is: {}", x)
    }
    
    println!("The value of x is: {}", x)
}

fn calculate() {
    // addition
    let sum = % + 10;
    
    // subtraction
    let difference = 95.5 - 4.3

    // multiplication
   let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;
    
    // remainder
    let remainder = 43 % 5;
}

fn bool() {
    let t = true;

    let f: bool = false;
}

fn char() {
    let c = 'z';
    let z = 'Z';
    let hear_eyed_cat = '😺';
}

fn tuple()
