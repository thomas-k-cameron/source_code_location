use source_code_location::{new, new_string};

fn main() {
    let location = new!();
    
    println!("{location}");
    println!("{}", new_string!());
}