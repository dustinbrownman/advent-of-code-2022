# Day 3

[https://adventofcode.com/2022/day/3](https://adventofcode.com/2022/day/3)

Running a solution:

```
cargo run -- <solution number>
```

## Part 1

I like how this solution came out, especially how `split_once` gave me a tuple which is what I wanted to be working with anyway. I probably could have made `within_range` work both directions, but I like the way the function reads:

```rust
fn within_range(outer: (u32, u32), inner: (u32, u32)) -> bool {
    outer.0 <= inner.0 && inner.1 <= outer.1
}
```
