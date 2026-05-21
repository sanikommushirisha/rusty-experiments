

pub fn guess_input() -> String {
    println!("Enter something!");

    let mut guess = String::new(); // = -> Means something to the variable. Right side => Value being bound to.
    //::new() -> associated function implemented on the String type.
    // variables are immutable by default. To make them mutable, use mut keyword before the variable name. utf-8 encoded text

    std::io::stdin().read_line(&mut guess).expect("Failed to read line"); // &mut guess -> references are immutable by default
    // So, we need to pass &mut to make it a mutable reference. 
    // Pass by reference
    // mutable reference allows us to change the value of the variable it points to


    //std::io::stdin().read_line(&mut guess) => returns a Result type, which is an enum that can be either Ok or Err variants.

    println!("You entered: {}", guess); 
    // {} is a placeholder for the value of guess

    // crate is a collection of Rust source code files
    // rusty-experiments is a binary crate, which means it is an executable program. Standalone
    // Library crate is intended to be used as a dependency in other projects. Not standalone

    //Crates.io is the rust community's crate registry.

    guess
}
