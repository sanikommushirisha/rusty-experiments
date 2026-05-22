pub fn own() {
    // Rust ownership lets you make memory safe guarentees without needing GC
    // Ownership: Set of rules on rust manages memory
    // Java GC: Automatically deallocates unused memory
    // C/C++: Manual memory management
    // Rust: Ownership system with rules and compile-time checks. If violated, RC errors

    // Stack & Heap
    // Data stored on stack should have known size at compile time. Store in heap otherwise
    // Heap -> You request certain amount of memory. A memory allocator finds that location and returns a pointer to it.
    // If you want to find the data -> follow the pointer -> memory address.
    // Slower as we follow pointer. 
    // Also, if we exceeds capacity, we reallocated: Allocate a new block of memory (usually double the current capacity) in a fresh, open area of the heap and update the pointer
    
    // When function executes, the arguments, functions local variables, pointers to heap data are pushed onto the stack. 
    // On end, all of that data is popped off the stack and freed.

    // Keep track of what data is on stack & what data is on heap 

    // Ownership rules:
    // Each value in rust has a variable (its owner).
    // One owner at a time.
    // When owner goes out of scope, value will be dropped (memory is freed).

    // Scope - Range within the program for which an item is valid.

    // String vs str
    // str is stored on stack and immutable. Fast & Effecient. 
    // String String is stored on heap & grow. It can store data where size is not known at compile time or can change.

    // Technical
    // With languages with GC, GC auto tracks & cleans memory
    
    // Without GC, you manually allocate & deallocate memory. 
    // If you forget to deallocate, you have a memory leak. If you deallocate twice, you error out.
    // If you deallocate too early, you have a dangling pointer. 
    
    // Rust deallocates memory when it goes out of scope(At closing curly bracket)
    let s1 = String::from("hello");
    let s2 = s1;
    // Rust invalidates the first variable(s1), instead of being called a shallow copy, it’s known as a MOVE.

    println!("s2 {}", s2);

    let mut s3 = String::from("hello");
    println!("s3 Before {}", s3);
    s3 = String::from("ahoy"); // s3 variable at L45 is out of scope and memory is freed right away

    print!("s3 {}", s3);
    // If we do want to deeply copy the heap data of the String, not just the stack data, Use clone

     // No clone needed as they are stored on stack and copy is fast. Both x and y are valid and independent variables.
    let x = 5;
    let y = x; // Integer implements copy trait, so it is copied rather than moved. 
    let z = x; // Both x and z are valid and independent variables.

    let y = 6;

    println!("x {}, y {}, z {}", x, y, z); // x 5, y 6, z 5

    // All the types that implement the Copy trait: int, floats, bool, char, and tuples that only contain these types.

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s); // s's value moves into the function...

    // println!("s {}", s); // RC error: borrow of moved value: `s` ->... and so is no longer valid here

    let x = 5; // x comes into scope
    makes_copy(x); // Because i32 implements the Copy trait,
    // x does NOT move into the function,so it's okay to use x afterward.


    // Returning values can also transfer ownership. 
    // Taking ownership and then returning ownership with every function is a bit tedious: a value without transferring ownership: references.

} // Here, x goes out of scope, then s. However, because s's value was moved,
  // nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.