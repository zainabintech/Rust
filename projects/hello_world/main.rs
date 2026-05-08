// hello_world / main.rs
// a program that prints "Hello, world!" to the screen.
//
// HOW TO RUN THIS FILE
//
// 1) open a terminal in this folder (D:\Rust\projects\hello_world)
// 2) compile:  rustc main.rs
//    -> this turns main.rs into an executable file (main.exe on windows)
// 3) run:      .\main          (windows)
//              ./main           (mac / linux)
//    -> this actually runs the program. you should see: Hello, world!
//
// remember: `rustc` only COMPILES. it does NOT run the program.
// running the produced executable is a separate step.


// PART 1 — the function declaration:  fn main()
//
//      fn   main   ()   {
//      ^    ^      ^    ^
//      |    |      |    |
//      |    |      |    +-- start of the function body (the code lives inside)
//      |    |      +------- parentheses: where parameters go (empty here = no inputs)
//      |    +-------------- the NAME of the function: "main"
//      +------------------- the keyword `fn` = "i'm declaring a function"
//
// why "main"?
//   every executable Rust program MUST have a function called `main`.
//   `main` is the entry point -- the first code that runs when you start
//   the program. without it, Rust does not know where to begin.
//
// why empty parentheses?
//   parameters are inputs a function takes. `main` here takes no inputs,
//   so the parentheses are empty: `()`.
//
// why a return type isn't written:
//   `main` returns nothing here. if a function returned something, you'd
//   write it like `fn main() -> i32 { ... }` (we'll see this later).
fn main() {

    // PART 2 — the body of main: what the program actually does
    //
    //      println!   (   "Hello, world!"   )
    //      ^          ^   ^                 ^
    //      |          |   |                 |
    //      |          |   |                 +-- closing parenthesis
    //      |          |   +-------------------- the ARGUMENT we pass in (a string)
    //      |          +------------------------ opening parenthesis: arguments go inside
    //      +----------------------------------- the macro name + `!`
    //
    // what is `println!` ?
    //   it's a MACRO that prints text to the screen and then adds a new line.
    //   "macro" = code that writes code at compile time. you don't need to
    //   understand the internals yet -- just know `println!` prints stuff.
    //
    // why the `!` after println ?
    //   the `!` tells Rust: "this is a macro, not a regular function."
    //   if you wrote `println(...)` (without `!`) Rust would look for a
    //   function called `println` and fail, because no such function exists.
    //
    // what is the `"Hello, world!"` part ?
    //   it's a STRING -- text wrapped in double quotes.
    //   we are PASSING this string as an ARGUMENT to `println!`.
    //   "passing an argument" just means: handing a value to a function/macro
    //   so it can use it. here the macro uses it by printing it.
    //
    // about the missing semicolon `;` :
    //   normally each statement in Rust ends with a semicolon `;` -- it tells
    //   Rust "this expression is finished, the next one starts now."
    //   here there is no `;` because this is the LAST line of `main`. Rust
    //   allows that, but it is more conventional to write:
    //       println!("Hello, world!");
    //   adding `;` makes it a clear statement ending. safe habit to keep.
    println!("Hello, world!")

} // <-- closing curly bracket: end of the `main` function body


// QUICK SYMBOLS RECAP (so future-me doesn't have to re-look these up)
//
//   fn          -> keyword to declare a function
//   main        -> special function name = the program's entry point
//   ( )         -> parentheses: where a function's parameters go
//   { }         -> curly brackets: wrap the function's body (its code)
//   println!    -> a macro that prints a line of text to the screen
//   !           -> means "this is a macro" (not a normal function call)
//   " "         -> double quotes mark the start and end of a string
//   ;           -> semicolon: ends an expression / statement
//   //          -> line comment: Rust ignores everything after `//` on the line
