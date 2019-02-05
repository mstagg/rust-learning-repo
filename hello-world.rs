/// This syntax generates libary docs.
/// This is the main function.
fn main() {
    /*
     * The statements here will be executed
     * when the compiled binary is called
     */

    // Print text to the console
    println!("Hello World!");
    println!("Im am a growing rustacean!");
    println!("I can {0} all {1} of my strings.", "template", 2);
    println!("I can also used {namedTemplate}", namedTemplate = "named templates");
    println!("I can format my numbers in binary ({0:b}) or hex ({1:X})", 12, 255);
    println!("I can even print pi to {0} decimals: {1:.*} ", 3, 3.14159);
}