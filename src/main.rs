mod file_read;
use file_read::FileRead;


fn main() {

    let file = FileRead::new("text.txt".to_string(), None).str_full_path();

    println!("{}", file);


}
