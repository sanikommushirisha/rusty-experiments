pub fn loops() {

    let count = 5;
    // Condition must be bool. unlike JS which allows truthy. Rust compiler(RC) will error otherwise: No implicit type conversion.
    if count > 5 { //rusty language - arms of if expression
        println!("Hey, I'm in greater than 5 loop!")
    } else {
        println!("Why is this girl coding an else loop at this stage!")
    }

    let is_greater = if count > 5 { true } else { false };
    println!("Yay, inline expression {}", is_greater);

    // if else statement all branches must return same time. RC will error otherwise.
    let complex_is_greater: bool = if count > 5 {
        println!("Count is greater than 5");
        true // Blocks evaluate to last expression
    } else {
        println!("Count is not greater than 5");
        false
    };
    println!("Yay, complex inline expression {}", complex_is_greater);

    loop {
        println!("This is an infinite loop!");
        break; // break statement to exit the loop
    }

    // Loop label to disambiguate between nested loops
    'outer: loop {
        let mut count = 0;
        println!("This is the outer loop!");

        loop {
            if count < 2 {
                count += 1;
                println!("Count is less than 2, skipping the rest of the inner loop. Count: {count}");
                continue; // Skips the rest of the inner loop and goes to the next iteration
            };
            println!("This is the inner loop! {}" , count);
            break 'outer; // Breaks out of the outer loop

        }
    }

    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("Pretty simple whole loop !!!");

    let arr = [10, 20, 30, 40, 50];

    for element in arr {
        println!("The value is: {element}");
    }

    for number in 2..4 { // Range from 2 to 4, exclusive of 4
        println!("{number}!");
    }

    let fh_test = convert_from_c_to_fh(25.0);
    println!("fh VALUE {}", fh_test)

}


fn convert_from_c_to_fh(x: f32) -> f32 {
    (9.0/5.0*x) + 32.0
}