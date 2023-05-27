use source_code_location::{new, SourceCodeLocation};

fn main() {
    println!("{:#?}", func());
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