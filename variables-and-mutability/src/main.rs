/* 
Applying the compiler directive to the entire file to avoid warnings for unused variables
This is useful when you are demonstrating concepts and may not use all variables defined
*/
#![allow(unused_variables)]

/*
Constants are always immutable and must have a type annotation
Constants can be defined in any scope, including the global scope
Constants are usually written in uppercase with underscores separating words
Constants have to ne known at compile time and can only be set to a constant expression
 */

const MAX_FRUITS: f64 = 100.0;

/*
Type Aliases allow you to create a new name for an existing type
Type Aliases are created using the 'type' keyword
*/

type Meters = i32;

// Compiler directives for all blocks below
 #[allow(unused_variables)]

fn main() {
    /*
    Any variables defined in side a block or scope is not accessible outside that block or scope.
    Variables are immutable by default in Rust. To make them mutable, use the 'mut' keyword.
    Variable shadowing allows you to redeclare a variable with the same name, creating a new variable.
     */
    let apples = 50;
    let oranges = 14 + 6;
    let fruits = apples + oranges;
    println!("I have {} apples.", apples);
    // The below formatting was introduced in Rust 1.5.x
    println!("I have {oranges} oranges.");
    println!("Total number of fruits in my garden: {}", fruits);
    println!(
        "I have {} apples and {} oranges which total {} fruits. I love that I have {2} fruits!",
        apples,  // 0 - Counts start from 0
        oranges, // 1
        fruits   // 2
    );
    println!("I have {apples} apples and {oranges} oranges which total {fruits} fruits.");

    // Making a variable Mutable
    let mut bananas = 30;
    println!("I have {bananas} bananas.");
    bananas = bananas + 20;
    println!("Now I have {bananas} bananas.");

    // Variable shadowing - This allows you to redeclare a variable with the same name
    let bananas = bananas + 10;
    println!("After some more bananas, I now have {bananas} bananas.");

    /*
    Scopes - A block inside curly braces creates a new scope inside the main function
    This would be considered an Inner Scope
    Variables defined inside this inner scope are not accessible outside of it
    Variables defined outside this inner scope are accessible inside it
    */
    {
        let bananas = bananas + 5;
        println!("In this inner scope, I have {bananas} bananas.");
    }

    println!("Outside the inner scope, I still have {bananas} bananas.");
    // Bring in the Constant that was defined at the top
    println!("The maximum number of fruits I can have is {MAX_FRUITS}.");

    // Using the Type Alias defined at the top
    #[allow(unused_variables)] // To avoid warning for unused variable (Compiler directive)
    // Only for the line below
    let mile_distance: Meters = 1609; // Using the Type Alias defined at the top
    println!("The distance in meters for one mile is {mile_distance} meters.");
    println!(
        "A marathons is 26.2 miles long which is equal to {} meters.",
        26.2 * mile_distance as f64
    );
}
