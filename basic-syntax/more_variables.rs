fn main() {
	// What could this value be?
	let unit = ();
    println!("The unit value: {:?}", unit); // :? is needed for formatting units

    // Copying an integer
    let mut my_integer = 10i8;
    let my_copied_integer = my_integer;
    println!("Successfully copied {}", my_copied_integer);

    // But what if we changed the original?
    my_integer = 20;
    println!("The original was changed to {}. The copied is {}", my_integer, my_copied_integer);
}
