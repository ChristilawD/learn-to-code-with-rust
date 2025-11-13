// Declare a `TOUCHDOWN_POINTS` constant at the file level set to the value 6.
const TOUCHDOWN_POINTS: i32 = 6;

fn main() {
    // Declare a `season` variable set to a string with your favorite season.
    let season: &str = "Fall";

    // Declare a `points_scored` variable set to 28.
    let mut points_scored: i32 = 28;

    // Update the team's score to 35.
    points_scored = 35;

    // Declare an `event_time` variable set to a string of "06:00".
    let event_time: &str = "06:00";

    // Use variable shadowing to redeclare `event_time` set to an integer of 6.
    let event_time: i32 = 6;

    // Print out all of the declared variables and constants.
    println!(
        "Season: {season}, Points Scored: {}, Touchdown Points: {2}, Event Time: {event_time}",
        points_scored, TOUCHDOWN_POINTS
    );

    // Declare a `favorite_beverage` variable set to a string of your favorite drink.
    #[allow(unused_variables)] // Compiler directive to silence unused variable warning
    let favorite_beverage: &str = "Coffee";
}
