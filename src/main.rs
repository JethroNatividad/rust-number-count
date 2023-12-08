use std::io;

// Create a program that prompts for an input string and dis- plays output that shows the input string and the number of characters the string contains.

// verbs: prompts, displays, shows
// nouns: input string, output, number of characters

// Input: input string
// Process: Count the number of characters
// Output: input string, and number of characters

// Test
// Input: Homer
// Expected Output: Homer has 5 characters


fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    println!("{} has {} characters", input.trim(), input.trim().chars().count());
    Ok(())
}
