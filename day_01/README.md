# Day 1

[https://adventofcode.com/2022/day/1](https://adventofcode.com/2022/day/1)

To run a solution, run the binary with the solution number you want. If you don't pass in any solution, it will default to the first.

```
./solution 1
```

## Part 1

I had to take some inspiration (straight copying) from the Rust docs to figure out how to iterate over a file line by line. Luckily, I think I'll be able to use that approach for many of these puzzles. Most of the rest was figuring out how to deal with results and optionals. I was really pleased with the `parse_line` function and it's ability to tell if a line had some content or none at all.

## Part 2

I knew I wanted to keep track of the top three "elf packs" as we iterated through rather than trying to sort them at the end, but I wasn't quite sure how to do that. I leaned on Copilot to help me with the syntax, especially uncovering the [`std::iter::Iterator position` method](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.position). That allowed me to insert a new pack into the correct position of the Vector and then `pop` off any that got pushed to the back.

Also, shout out to [this blog post](https://mkaz.blog/working-with-rust/command-line-args/) that helped me sort my command line arguments. I wanted to be able to run both solutions and this was the most staight-forward example I could find.
