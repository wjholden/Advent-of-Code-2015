# Advent of Code 2015

## Daily Themes and Stars

1. `**` Basic programming: loops, conditions, variables, and tests.
2. `**` Splitting strings, 2D arrays, parsing integers from strings, arithmetic.
3. `**` Hashmaps (comparable objects), sets, refactoring

## Lessons Learned

* I really liked `go run` and how Go made it so easy to build an executable from
a `.go` file, but I never got comfortable with `go install` and `go get` from Github.
`rustc` can compile a Rust source file directly, which is fine.
* A cool strength Rust has over Go is that a package may contain
[multiple binary crates](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html)
in the `src\bin` directory. This is the structure I'm using for Advent of Code.
* Calls to `dbg!` and `assert!` transfer ownership of the arguments. This means
you need to pass references (`&`) instead of values.

## Documentation

Enter the command `rustup doc` to access the Rust Programming Language ("the book"), Rustonomicon, examples, API documentation, and a lot more.
Examples:

```
rustup doc std::collections
rustup doc for
```

* [The Rust Programming Language](https://doc.rust-lang.org/book/)
* [Effective Rust](https://www.lurklurk.org/effective-rust/)
