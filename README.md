# Source Code Location
It prints out the locatin on the source code.


```rust
use source_code_location::{new};

fn main() {
    let location = new!();
    println!("{location}");
    // [examples/example.rs] 4:20
}
```