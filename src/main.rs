mod file_read;
use file_read::FileRead;

use notify::Result;


fn main() -> Result<()>{

    let file = FileRead::new("livefile.txt".to_string(),Some("/tmp/".to_string()));



    file.watch()

}
