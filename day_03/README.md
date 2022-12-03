# Day 3

[https://adventofcode.com/2022/day/3](https://adventofcode.com/2022/day/3)

Running a solution:

```
cargo run -- <solution number>
```

## Part 1

Well, that was a chore. I knew I wanted to use some kind of `Set` so I could easily tell what the two compartments had in common with an `intersection`, but the ergonomics of that was really tough. Luckily the compiler helped guide things, though most of my confusion was around just not understanding the fundamentals of Rust.

I tried to use a Macro I'd seen, `include_str!` to make reading the file easier, but I must have missed a crucial step because it just made it _really_ hard to work with each line. I ended up going back to the old `read_lines` method. I might adjust that some so that the solution functions don't have to handle the `Result` objects.
