# Advent of Code 2015

## Daily Themes and Stars

1. `**` Basic programming: loops, conditions, variables, and tests.
2. `**` Splitting strings, 2D arrays, parsing integers from strings, arithmetic.
3. `**` Hashmaps (comparable objects), sets, refactoring
4. `**` Embarrassingly parallelizable problems, refactoring, use of existing libraries
5. `**` Strings, unit testing, regular expressions
6. `**` Mutable state, object-oriented programming, procedural programming, polymorphism, abstraction
7. `**` Parsing, dynamic programming, $O(2^n)$
8. `**` String encoding, look-ahead parsers (peeking)
9. `**` Graphs, permutations
10. `**` Automata, peeking, integer sequences
11. `**` Password complexity, radix overflow, test cases, trim your inputs
12. `**` JSON. This is going to be too difficult in Rust, so I'm using Julia instead.
13. `**` [Circular permutations](https://mathworld.wolfram.com/CircularPermutation.html). Also, now I have a parser (nom).
14. `**` Don't be clever, OOP, mutability
15. `**` Linear algebra, gradient descent, constrained optimization
16. `**` Constraint satisfaction
17. `**` Superficial Knapsack problem, combinatorics
18. `**` Conway's Game of Life, parallelization surprisingly doesn't help
19. `**` [Grammars](https://www.reddit.com/r/adventofcode/comments/3xflz8/comment/cy4etju/#s), not actually a trie problem
20. `**` [Integer sequences](https://oeis.org/A000203)
21. `**` RPG, combinatorics (surprisingly small), functional approach works better than expected
22. `**` Another RPG! Combinatorics with larger $n$, BFS works with greedy approach where DFS is intractable.
23. `  ` 
24. `  ` 
25. `  ` 

## Lessons Learned

* I really liked `go run` and how Go made it so easy to build an executable from
a `.go` file, but I never got comfortable with `go install` and `go get` from Github.
`rustc` can compile a Rust source file directly, which is fine.
* A cool strength Rust has over Go is that a package may contain
[multiple binary crates](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html)
in the `src\bin` directory. This is the structure I'm using for Advent of Code.
* Calls to `dbg!` and `assert!` transfer ownership of the arguments. This means
you need to pass references (`&`) instead of values.
* Tests in Rust are excellent! Use `cargo test` for all your testing needs. See day 4.
* I would be interested to go back to day 4 to do this in parallel. This would have been easy in Go.
* Looks like the compiler will (at least, by default) completely ignore errors (including syntax errors) in your tests.
* Like Go, you can declare a variable in [`if let`](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html). For example: `if let Error(e) = f(x) { g(e); }`.
* You can also `while let`, which avoids an implicit move of the iterator inside a `for` loop (see day 8).
* Lifetime annotations might be necessary when mutating hash maps in a function.
* The `memoize` library was not as effortless as `@cache` in Python.
* [Rayon](https://docs.rs/rayon/latest/rayon/), on the other hand, is amazing! You should be able to replace `iter` with `par_iter` for easy parallelization.
* `String` and `&str` probably made my day 11 solution significantly slower than it would have been on a byte array.
Maybe next time you have a string problem like this, consider `Vec<u8>` for in-place character mutation.
* [jlrs](https://github.com/Taaitaaiger/jlrs) is a potential means of interfacing Julia with Rust,
but it looks pretty complicated and I found it easier to just invoke the command.
* A surprise with invoking the command is that you don't want to provide input as an argument.
Luckily, Julia's `JSON3.read` can automatically guess that short input strings are filenames.
* I like using VSCode more than RustRover, but RustRover has a better debugger.
* [nalgebra](https://www.nalgebra.org/) is fast and usable.
* Multithreading with Rayon is very cool, but it might not always help. In day 15, it actually made things slower.
* You can separate `match` cases to the same arm with `|`.
* Looks like Rust does have Java-style method references. For example, `map(Vec::len)`.

## Documentation

Enter the command `rustup doc` to access the Rust Programming Language ("the book"), Rustonomicon, examples, API documentation, and a lot more.
Examples:

```
rustup doc std::collections
rustup doc for
```

* [The Rust Programming Language](https://doc.rust-lang.org/book/)
* [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
* [Effective Rust](https://www.lurklurk.org/effective-rust/)
* [From Julia to Rust](https://miguelraz.github.io/blog/juliatorust/)
