

pub fn variables() {
    let x = 5;
    //x = 6; Compile time errors when we try to change the value of an immutable variable
    // Adding mut makes the variable mutable and indicated that the programmer intends to change the value of the variable
    println!("{}", x);

    let x = x + 1; // Second variable overshadows the first;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x in the outer scope is: {x}"); // After the inner scope ends, inner shadowing ends and value is back to 6
    // Shadowing is different from mutating. 
    // Shadowing allows to perform a few transformations on a value & variable is immutable after transformations are complete;
    // must use `let` to declare a new variable that shadows the previous variable.

    let spaces = "   ";
    let spaces = spaces.len(); // Shadowing: Same name for different variables & different type. No naming headache


    const TEST: i32 = 6; // Constants are bound to a name and always immutable. no mut support. Type must always be annotated.
    const TEST1 : i32 = 65 * 32; // Constant expression:  evaluated at compile time, no support for run time evaludated expressions
    // Valid for the entire time a program runs: Within the scope they are declared

    // Rust Datatypes
    // Statically typed language. It needs to know data type at compile time. Inferred by default but can be annotated.

    let guess: u32 = "42".parse().expect("Not a number!");
    // When many types are possible, the compiler will complain and forces you to declare types.

    // Rust scalar types: Represent a single value. Integers, floating-point numbers, Booleans, characters.
    // Integers:
    // Each signed variant can store numbers from −(2n − 1) to 2n − 1 − 1 inclusive,
    // u8 that can hold values between 0 and 255. If outside, integer overflow 
    // In debug mode, Rust can check run time errors for integer overflow 
    // Floating point: f32 and f64. Default is f64.
    // Char: represents a single Unicode scalar value. 4 bytes in size. Can represent a lot of characters.
    // Boolean: true and false. 1 byte in size.

    // Rust Compound types: Can group multiple values into one type. Tuple and Array.

    // Tuple: A general way of grouping together a number of values with a variety of types into one compound type.
    let t: (i32, f32) = (500, 6.4);
    let (x, y) = t; // Destructuring a tuple into individual variables
    print!("Tuple: {}, {}", t.0, t.1); // Accessing tuple values with a dot and index
    print!("Destructured: {}, {}", x, y); // Accessing destructured values

    // Array: A collection of values of the same type. Fixed length. Stored on the stack.
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5]; // Shorthand for creating an array with the same value repeated. [3, 3, 3, 3, 3]
    println!("Array: {:?}, {:?}", a, b); // Debug print for arrays

    // print!("{}", a[10]) // Rust checks array bounds at runtime and will panic if we try to access an index outside the array's length.
    // Memory safety: In other languages, this might lead to undefined behavior(access invalid memory), 
    // but Rust will stop execution and report an error.

    // Functions
    another_function(5,6,);
    //snake case: convention for function and variable names. All lowercase with underscores between words.
    // curly brackets: function body
    // Function has parameters - x, y and arguments - 5, 6. 
    // Parameters are variables & arguments are the actual values passed to the function
    // We must declare the type of each parameter.

    // Function body consists of statements and expressions. 
    // Statements perform an action and do not return a value. Expressions evaluate to a value.
    // The let y = 6 statement does not return a value

    let z = {
        let x = 3; // Statement: does not return a value
        x + 1  // no semi-colon after an expression because it would turn it into a statement and not return a value.
    };
    // The above block is an expression that evaluates to the value of the last expression in the block
    print!("Value of z: {z}");
    // Expressions evaluate to a value and make up most of the rest of the code. 
    // Calling a function is an expression. Calling a macro is an expression.
    // Return value of a function is a value returned by the last expression in the function body
   
}

// Always declare return type of a function.
fn another_function(x: i32, y: i32) -> i8 {
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    5 
}