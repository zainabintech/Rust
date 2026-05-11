### Creating a New Project with Cargo
cargo new project_name

What it does
- Creates a new Rust project folder
- Inside the folder you get:

        Cargo.toml → project configuration file
        src/main.rs → main source code file

### Running the Project
cargo run

What it does
- Compiles the program
- Runs the program immediately
- You must be inside the project folder when running this command

### Using the Standard Library (Input/Output)
use std::io;

- This imports Rust’s input/output tools so you can use them in your program.

Breakdown
    - std = standard library (built-in Rust tools)
    - io = input/output module

What it is used for
    - Getting user input from the keyboard
    - Printing output
    - Handling basic interaction with the user

### Creating Variables
let guess;

- let is used to create a variable
- By default, variables in Rust are immutable
- You cannot change their value after setting them

### Mutable Variables
let mut guess;

- mut means the variable is changeable

So:

Without mut → cannot change value
With mut → can update value later

### Strings and String::new()
let mut guess = String::new();

What is String?
- A data type used to store text
- Unlike fixed text, it can grow and change

What is String::new()?
- It creates a new empty String.

What does :: mean?
- “access something that belongs to a type”

So:

new() is a function that belongs to the String type

- String is like a factory
- new() is the function that creates a fresh empty string

### Getting User Input
io::stdin()

- Gives access to keyboard input (standard input)

### Reading User Input
io::stdin().read_line(&mut guess)

This line:

- Takes user input from the keyboard
- Stores it inside the variable guess

### Understanding References (&)
&mut guess

A reference means:

- You are not copying the value
- You are borrowing it

Why references exist

- If Rust copied everything:

    programs would use more memory
    programs would be slower

So instead:

Rust allows safe borrowing of data

Simple analogy

guess = a notebook
&guess = pointing to the notebook instead of copying it

What &mut means
&mut guess

& = borrow the value (reference)
mut = allow modification

- You are borrowing the variable and allowing it to be changed.

### Why guess must be mutable
let mut guess = String::new();

Reason:

.read_line() modifies the string
It adds user input into it

If it was not mutable:

Rust would prevent the change for safety reasons

### What read_line() Does
io::stdin().read_line(&mut guess)

- Reads what the user types
- Stores it inside guess
- read_line() does NOT just return text.

It returns a result:

Result<usize, std::io::Error>

### What is Result?

- Result tells you whether something succeeded or failed.

### Result is an Enum
enum Result {
    Ok(value),
    Err(error),
}


### What is an Enum?

An enum is a type that can be one of multiple possible values.

Example (Traffic Light)

A traffic light can be:

Red
Yellow
Green

Only one at a time

That is an enum.

### What is a Variant?

A variant is one possible value inside an enum.

So in Result:

Ok is one variant (success)
Err is another variant (failure)

### What read_line() returns
Ok(value) → input succeeded
Err(error) → something went wrong

### Full Mental Model

Think of it like this:

String::new() → empty box
guess → the box storing input
stdin() → keyboard input
.read_line() → puts user input into the box
&mut guess → allows safe writing into the box
Result → tells if it worked or failed

### Final Summary
cargo new → create project
cargo run → compile and run
use std::io → input/output tools
let → create variable
mut → allow change
String::new() → create empty string
:: → access function inside a type
stdin() → get keyboard input
read_line() → read user input
&mut → borrow and allow modification
Result → success/failure type
enum → type with multiple possible values
variant → one option inside an enum