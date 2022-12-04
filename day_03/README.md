# Day 3

[https://adventofcode.com/2022/day/3](https://adventofcode.com/2022/day/3)

Running a solution:

```
cargo run -- <solution number>
```

## Part 1

Well, that was a chore. I knew I wanted to use some kind of `Set` so I could easily tell what the two compartments had in common with an `intersection`, but the ergonomics of that was really tough. Luckily the compiler helped guide things, though most of my confusion was around just not understanding the fundamentals of Rust.

I tried to use a Macro I'd seen, `include_str!` to make reading the file easier, but I must have missed a crucial step because it just made it _really_ hard to work with each line. I ended up going back to the old `read_lines` method. I might adjust that some so that the solution functions don't have to handle the `Result` objects.

## Part 2

While I'm sure there's a cleaner way to do it, I liked using `lines.next()` to grab the next 3 lines. It avoids needing to load the entire file into an iterable so it can be chunked. I also spent a _lot_ of time debugging why my sets were coming out empty. I knew I was going to need to reset the current group variable, but before I did that I wanted to confirm that the common priorities were coming out as expected. By not clearing it out though, I was iterating over an ever-growing list many times and thus getting empty results. I finally realized my mistake and fixed it and everything worked as I originally intended!
