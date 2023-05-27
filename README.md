# Source Code Location
It prints out the location on the source code.


## Example
### Printing the location
```rust
use source_code_location::{new};

fn main() {
    let location = new!();
    println!("{location}");
    // [examples/example.rs] 4:20
}
```

### Location as struct
```rust
use source_code_location::{new, SourceCodeLocation};

fn main() {
    println!("{:#?}", func());
    /*
    Err(
        SourceCodeLocation {
            file_name: "examples/struct.rs",
            line_number: 11,
            column_number: 17,
        },
    )
    */
}

fn func() -> Result<(), SourceCodeLocation>{
    let something = true;
    let _data = {
        if something {
            Err(new!())?
        } else {
            true
        }
    };
    Ok(())
}
```


### Location as string
```rust
use source_code_location::{new_string};

fn main() {
    println!("{:#?}", func());
}

fn func() -> Result<(), &'static str>{
    let something = true;
    let _data = {
        if something {
            Err(new_string!())?
        } else {
            true
        }
    };
    Ok(())
}
```