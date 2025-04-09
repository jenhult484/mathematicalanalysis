// Example Rust code to demonstrate some mathematical concepts.
fn main() {
    // Addition of two numbers
    let num1 = 5;
    let num2 = 3;
    println!("The sum of {} and {} is {}", num1, num2, num1 + num2);

    // Subtraction of two numbers
    let num3 = 7;
    let num4 = 2;
    println!("The difference between {} and {} is {}", num3, num4, num3 - num4);

    // Multiplication of two numbers
    let num5 = 6;
    let num6 = 4;
    println!("The product of {} and {} is {}", num5, num6, num5 * num6);

    // Division of the first number by the second number (if not divisible, returns a floating point result)
    if num1 % num2 == 0 {
        let quotient = num1 / num2;
        println!("The division between {} and {}, resulting in an integer quotient is {}", num1, num2, quotient);
    } else {
        let remainder = num1 - (num1 % num2);
        println!("The division between {} and {}, resulting in a decimal quotient: {}", num1, num2, remainder);
    }
}
