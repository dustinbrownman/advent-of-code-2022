# Day 5

Running a solution:

```
cargo run -- <solution number>
```

## Part 1

Sometimes you think you're taking a shortcut by not manually parsing a block of text to put it into the data structure you want but then you spend 40 minutes debugging to figure out why you go the wrong answer and it's because you didn't input the text correctly. But at least the solution was pretty straightforward, and it was nice using `Struct`.

```rust
struct CargoShip {
    stacks: [Vec<&'static str>; 9],
}

// implement a move function on CargoShip that takes a tuple of i8
impl CargoShip {
    fn move_crate(&mut self, (amount, from, to): (i8, i8, i8)) {
        for _ in 0..amount {
            let this_crate = self.stacks[(from - 1) as usize].pop().unwrap();
            self.stacks[(to - 1) as usize].push(this_crate);
        }
    }
}
```
