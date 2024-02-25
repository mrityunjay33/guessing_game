fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // will give error - doesn't works like javascript
    // if number {
    //     println!("number was three");
    // }

    // Using if in a let Statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
    
    // error (both the if arm and the else arm types are mismatched)
    // let number = if condition { 5 } else { "six" }; 


}