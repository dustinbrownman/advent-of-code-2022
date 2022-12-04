# Day 4

[https://adventofcode.com/2022/day/4](https://adventofcode.com/2022/day/4)

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

## Part 2

Creating the overlaps function was harder than I thought it would be. I eventually realized I needed to see which range was the "higher" one for the overlapping logic to work.

```rust
fn overlaps(a: (u32, u32), b: (u32, u32)) -> bool {
    let (a_lower, a_upper) = a;
    let (b_lower, b_upper) = b;

    if b_upper >= a_upper {
        a_upper >= b_lower
    } else {
        b_upper >= a_lower
    }
}
```
