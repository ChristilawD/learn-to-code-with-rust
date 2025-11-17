fn main() {
    /*
    Samples of various integer types in Rust
    Underscore separators are used for better readability
    Underscores do not affect the actual value
    isize and usize are pointer-sized integers
    usize is used for indexing and memory size representationHello
    */
    let eight_bit: i8 = -112;
    let eight_bit_unsigned: u8 = 244;
    let sixteen_bit: i16 = -32_768;
    let sixteen_bit_unsigned: u16 = 65_535;
    let thirty_two_bit: i32 = -2_147_483_648;
    let thirty_two_bit_unsigned: u32 = 4_294_967_295;
    let sixty_four_bit: i64 = -9_223_372_036_854_775_808;
    let sixty_four_bit_unsigned: u64 = 18_446_744_073_709_551_615;
    let one_twenty_eight_bit: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728;
    let one_twenty_eight_bit_unsigned: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    let pointer_sized: isize = -9_223_372_036_854_775_808;
    let pointer_sized_unsigned: usize = 18_446_744_073_709_551_615;
    println!("8-bit signed: {}", eight_bit);
    println!("8-bit unsigned: {}", eight_bit_unsigned);
    println!("16-bit signed: {}", sixteen_bit);
    println!("16-bit unsigned: {}", sixteen_bit_unsigned);
    println!("32-bit signed: {}", thirty_two_bit);
    println!("32-bit unsigned: {}", thirty_two_bit_unsigned);
    println!("64-bit signed: {}", sixty_four_bit);
    println!("64-bit unsigned: {}", sixty_four_bit_unsigned);
    println!("128-bit signed: {}", one_twenty_eight_bit);
    println!("128-bit unsigned: {}", one_twenty_eight_bit_unsigned);
    println!("Pointer-sized signed: {}", pointer_sized);
    println!("Pointer-sized unsigned: {}", pointer_sized_unsigned);

    /*
    Strings in Rust are UTF-8 encoded by default
    String literals are immutable
    */
    let greeting: &str = "Hello, Rust!";
    println!("{}", greeting);
    /*
    Strings when asking for user input are mutable and dynamically sized
    Dynamic strings are created using the String type
    */
    let mut user_input: String = String::new();
    println!("\tPlease enter some \"text:\"");
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    println!("You entered: {}", user_input.trim());

    // Raw string literals do not process escape sequences
    let raw_str: &str = r#"This is a raw string.\nNo escape sequences are processed."#;
    println!("{}", raw_str);

    // Methods on integers and strings
    let value: i32 = -15;
    println!("Integer value: {}", value.abs());
    let empty_space = "   Trim me!   ";
    println!("Before trim: '{}'", empty_space);
    println!("After trim: '{}'", empty_space.trim());
    println!("{}", value.pow(4));

    // Floating point types in Rust are signed by default
    let float_32: f32 = 3.14159;
    let float_64: f64 = 2.718281828459045;
    println!("32-bit float: {}", float_32);
    println!("64-bit float: {}", float_64);

    // To Cast between types, use the 'as' keyword
    let int_value: i32 = 42;
    let float_value: f64 = int_value as f64;
    println!("Integer: {}, as float: {}", int_value, float_value);

    // Augmented Assignment Operators - Must be mutable variables
    let mut aug_value: i32 = 10;
    aug_value += 5; // aug_value = aug_value + 5
    println!("After += 5: {}", aug_value);

    // This will return a boolean value of True
    println!("{}", 13 == 10 + 3);

    // UTF stands for Unicode Transformation Format
    let first_initial: char = 'D';
    let first_name: &str = "Dennis";
    let last_initial: char = 'C';
    let last_name: &str = "Christilaw";
    let imoji_face: char = '🤗'; // In Windows use Windows Key + . to access emoji panel
    println!("Full Name: {} {}", first_name, last_name);
    println!("Initials: {}.{}.", first_initial, last_initial);
    println!("Imoji Face: {}", imoji_face);

    // Arrays in Rust have a fixed size - Of the same type
    let number_array: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("First element: {}", number_array[0]); // Index Counts start at 0
    println!("Array length: {}", number_array.len());

    let apples = ["Granny Smith", "Fuji", "Gala", "Honeycrisp"];
    println!("I like {} apples.", apples[2]);
    println!("Total apple types: {}", apples.len());

    // Reading and Writing array elements
    let mut mutable_array: [i32; 4] = [1, 2, 3, 4];
    println!("Original array: {:?}", mutable_array);
    // You can only replace elements in a mutable array
    mutable_array[2] = 10;
    println!("Modified array: {:?}", mutable_array);

    // Display Traits - Cannot be used on collections like arrays
    let display_number: i32 = 255;
    println!("\nDefault Display: {}", display_number); // Default display
    println!("Binary: {:b}", display_number); // Binary format
    println!("Hexadecimal: {:x}", display_number); // Hex format
    println!("Octal: {:o}", display_number); // Octal format

    // Call the debug trait on a collection using {:?}
    let debug_array: [i32; 5] = [5, 10, 15, 20, 25];
    println!("Debug Array: {:?}", debug_array);
    println!("Debug Array (pretty): {:#?}", debug_array);
    println!("Debug Array another way: {debug_array:?}");
    println!("Debug Array pretty another way: {debug_array:#?}");

    // This will use the pretty print macro to display the array
    dbg!(&debug_array);

    // Tuples in Rust can hold multiple types
    let person: (&str, i32, char) = ("Alice", 30, 'A');
    println!("Name: {}", person.0); // Accessing first element
    println!("Age: {}", person.1); // Accessing second element
    println!("Grade: {}", person.2); // Accessing third element

    let (student_name, student_age, student_grade) = person; // Destructuring the tuple
    println!("Student Name: {}", student_name);
    println!("Student Age: {}", student_age);
    println!("Student Grade: {}", student_grade);

    // Ranges in Rust are used to represent a sequence of values
    let range_inclusive = 1..=5; // Inclusive range from 1 to 5
    let range_exclusive = 1..5; // Exclusive range from 1 to 5
    println!(
        "Inclusive Range: {:?}",
        range_inclusive.collect::<Vec<i32>>()
    );
    println!(
        "Exclusive Range: {:?}",
        range_exclusive.collect::<Vec<i32>>()
    );

    // Iterating over a range
    for number in 1..=5 {
        println!("Number in range: {}", number);
    }
}
