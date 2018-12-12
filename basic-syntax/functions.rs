fn say_hello() {
    println!("Hello!");
}

fn count_hellos(times: u8) {
    println!("Printing hello {} times", times);
    for i in 1..times + 1{
    	print!("{}: ", i);
    	say_hello();
    }
}

// Function that returns a str(ing)
fn get_something() -> u8 {
    return 9;
}

fn main() {
    // Call another function to say hello
    say_hello();

    // Pass an argument to a function
    count_hellos(3);

    // Print out the value returned from a function
    println!("I got a {} from a function!", get_something());
}
