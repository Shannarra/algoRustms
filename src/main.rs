use std::io::{self, Read, BufRead};

///
/// ```rust
/// assert_eq!(read_contents(".gitignore"), "\".gitignore\" file contents: \n/target\n")
/// ```
fn read_contents(file: &str) -> String {
    match std::fs::read_to_string(file) {
        Ok(content) => format!("\"{}\" file contents: \n{}", file, content),
        Err(e) => panic!(e)
    }
}

fn rust_in(result: &mut String) {
   for line in io::stdin().lock().lines() {
       return result.push_str(&*line.unwrap())
   }
}

fn main() -> std::io::Result<()>{

    let mut input = String::new();

    rust_in(  &mut input);
    println!("{}", read_contents(&input));
    assert_eq!(read_contents(".gitignore"), "\".gitignore\" file contents: \n/target\n");
    Ok(())
}
