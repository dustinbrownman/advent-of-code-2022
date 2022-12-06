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

## Part 2

Well that was much easier, and a good chance to show off the strength of `VecDeque`! There might be more efficient ways of grabbing multiple elements off a stack at once, but I like how the code flow feels like the crane is picking up multiple crates and then moving them together.

```rust
impl CargoShip {
    //...
    fn super_move_crates(&mut self, (amount, from, to): (i8, i8, i8)) {
        let mut crane = VecDeque::new();
        // crane loads up the crates
        for _ in 0..amount {
            let this_crate = self.stacks[(from - 1) as usize].pop().unwrap();
            crane.push_front(this_crate);
        }

        // then drops them off on the stack
        for _ in 0..amount {
            let this_crate = crane.pop_front().unwrap();
            self.stacks[(to - 1) as usize].push(this_crate);
        }
    }
}
```
