# rust-hist (hist) - v0.1.0
A tiny library to print histograms in Rust.

# Examples
#### Print a histogram.
```rust
let mut h = hist::Hist::new(30, 5, &vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], &vec![21, 10, 15, 17, 26, 8, 12, 2, 5, 7]);

h.display();
```

or just:
```rust
hist::Hist::new(30, 5, &vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], &vec![21, 10, 15, 17, 26, 8, 12, 2, 5, 7]).display();
```

##### Result

```bash
              *
*         *   *
*      *  *   *     *
*   *  *  *   *  *  *         *
*   *  *  *   *  *  *      *  *
*******************************
```

# Installation

Add this line to your Cargo.toml:

```toml
[dependencies]
hist = "0.1.0"
```

and then add this line to your main.rs:

```rust
extern crate hist;
```
