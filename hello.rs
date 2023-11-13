// This is a comment, and is ignored by the compiler.
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter"
// shortcut.

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function.
fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    println!("Hello World!");

    display_square_roots()
}


fn display_square_roots() {

    println!("display sqare roooots");
}

fn newton_rooter(a: f32, n: f32) -> f32 {
    let mut k = a / 2.0 + n / a;
    return k;
}