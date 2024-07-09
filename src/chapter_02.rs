// this is a submodule
pub mod programming_a_guessing_game {
    // required for user input
    use std::io;

    pub fn guessing_game() {
        // prints something, `String` in this case
        println!("Guess the number!");
        println!("Please input your guess: ");

        // creates mutable variable `guess` and binds it to a `new`, empty instance of a `String`
        let mut guess = String::new();
        io::stdin()
            // call the read_line method on the standard input handle to get input from the user
            // and store it in a mutable reference to `guess`
            .read_line(&mut guess)
            // `read_line` returns an `enum` of `Result` type which in turn returns either `Ok()`
            // if the operation was successful or `Err()` if it failed.  If this instance of `Result`
            // is an `Ok` value, `expect` will take the return value that `Ok` is holding and return
            // just that value so that it can be used, in this case the users input. In case
            // `read_line` returns an `Err()`, the program will panic and crash printing the lines,
            // "Failed to read line"
            .expect("Failed to read line");

        println!("You guessed: {}", guess);
    }
}