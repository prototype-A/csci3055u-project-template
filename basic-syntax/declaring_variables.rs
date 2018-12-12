fn main() {
    // Variables can always by type annotated
    let my_boolean = true;
    let my_annotated_boolean: bool = true;	// Annotated

    // The two below applies to unsigned integers and floating-point values as well
    let my_integer: i32 = 5;				// Default annotation
    let my_suffix_annot_integer = 5i32;		// Suffix annotation

    // Variables must be declared mutable in order to be able to change their value
    let mut my_mutable_variable = 10;
    println!("My mutable variable is {}", my_mutable_variable);			// Starts out as 10
    my_mutable_variable = 11;
    println!("My mutable variable is now {}", my_mutable_variable);		// Now 11

    // Variable type can be changed with "shadowing"
    let my_mutable_variable = true;
    println!("And now, my mutable variable is now {}", my_mutable_variable);	// Now a boolean with the value of true
}
