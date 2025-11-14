/*
Declare an i32 variable assigned to 1337.
Use the underscore character to add a visual
separator between the numbers.

Cast the i32 to an i16 integer and assign the result
to a separate variable.

Declare a floating-point value of your choosing.
Print out the number with 3 digits of precision.

Declare a 'with_milk' variable set to a Boolean.
Declare a 'with_sugar` variable set to a Boolean.

Declare a 'is_my_type_of_coffee` variable. It should
be set to true if the coffee has both milk and sugar.

Declare an `is_acceptable_coffee` variable. It should
be set to true if the coffee has either milk or
sugar.

Declare an array with four i8 integers of your choosing
Print out the array in its Debug representation.

Declare a tuple consisting of the integer, float,
a Boolean, and the array that you previously declared.
Print out the tuple in its Debug representation.
*/
fn coding_challenge() {
    let my_integer: i32 = 1_337;
    let my_integer_i16: i16 = my_integer as i16;

    let my_float: f64 = 42.987654321;
    println!(
        "Floating point number with 3 digits of precision: {:.3}",
        my_float
    );

    let with_milk: bool = true;
    let with_sugar: bool = false;

    let is_my_type_of_coffee: bool = with_milk && with_sugar;
    let is_acceptable_coffee: bool = with_milk || with_sugar;

    let my_array: [i8; 4] = [10, 20, 30, 40];
    println!("Array in Debug representation: {:?}", my_array);

    let my_tuple: (i32, f64, bool, [i8; 4]) =
        (my_integer, my_float, is_my_type_of_coffee, my_array);
    println!("Tuple in Debug representation: {:?}", my_tuple);
}
