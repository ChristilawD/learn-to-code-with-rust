fn main() {
    open_store("Saint George");
    bake_pizza(50, "Pepperoni");
    swim_in_profits();
    swim_in_profits();
    swim_in_profits();
    open_store("Ivans");
    bake_pizza(20, "Sausage");
    let result = square(5);
    println!("The square of 5 is {result}");
    let result = square(9);
    println!("The square of 9 is {result}");
    let result = square2(5);
    println!("The square of 5 is {result}");
    let result = square2(9);
    println!("The square of 9 is {result}");
    let unit = empty_tuple();
    println!("The value of the unit tuple is: {:?}", unit);
    scopes();
}

fn open_store(neighborhood: &str) {
    println!("Opening my Pizza Shop in {neighborhood}!");
}

fn bake_pizza(number: i32, topping: &str) {
    println!("Baking {number} {topping} pizzas!");
}

fn swim_in_profits() {
    println!("Swimming in profits!");
}

fn square(number: i32) -> i32 {
    return number * number;
}

// implicit return
fn square2(number: i32) -> i32 {
    number * number
}

// A Unit function - returns the empty tuple
fn empty_tuple() -> () {
    ()
}

fn scopes() {
    let a = 5;
    {
        let b = 10 * a;
        println!("a: {a}, b: {b}");
    }
    // println!("a: {a}, b: {b}"); // This would cause a compile-time error
} 