# Day 2

[https://adventofcode.com/2022/day/2](https://adventofcode.com/2022/day/2)

I decided to use `cargo` to take advantage of the Rust analyzer. It makes the file structure more complicated, but also running the program is simpler since you don't _have_ to export the binary.

```
cargo run -- <solution number>
```

## Part 1

This was a nice chance to work with tuples. I really wanted to be able to match my tuple against the items in the winning/drawing vectors, but had to settle for a simple `if` statement.

I also had a hard time dealing with the `Result` object coming back for each line. The analyzer kept yelling at me about using `if let` and then saying it wasn't necessary. What I figured out was that I needed an `else` conditional on the `if let` for it to stop warning me.

## Part 2

This wasn't two bad. I just needed to invert the logic a bit. It was hard to get my head around how to structure the `play_rock|paper|scissors` vectors, but still think it's a nice way to handle it (versus needing to do a bunch of nested conditionals).
