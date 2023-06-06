use std::io;

fn main() {
    println!("Enter the number of rows for the pyramid:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let rows: u32 = input.trim().parse().expect("Invalid input. Please enter a positive integer.");

    let mut row = 1;
    let mut spaces = rows as i32 - 1;
    let mut stars = 1;

    while row <= rows {
        let mut space_count = spaces;
        let mut star_count = stars;

        // Print spaces
        while space_count > 0 {
            print!(" ");
            space_count -= 1;
        }

        // Print stars
        while star_count > 0 {
            print!("*");
            star_count -= 1;
        }

        println!(); // Move to the next line

        row += 1;
        spaces -= 1;
        stars += 2;
    }
}
