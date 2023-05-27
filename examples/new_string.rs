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