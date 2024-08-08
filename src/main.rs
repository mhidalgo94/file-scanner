mod file_read;

use notify::Result;
use file_read::FileRead;
use std::path::{Path, PathBuf};
use std::env;


fn main() -> Result<()>{

    // Collect the arguments skip the first argument (program name)
    let args: Vec<String> = env::args().skip(1).collect();

    // Collect the rest as path
    let paths : Vec<PathBuf> =  args.iter().map(PathBuf::from).collect();


    

    if paths.is_empty() {
        let file = FileRead::new("".to_string(),None);
        let str_format = format!("Watching local folder default program ({})", file.str_full_path());
        println!("{}", str_format);
        file.watch()?

    } else {
        // println!("Folder and Files: {:?}", &args);
        for file_path in paths {
            let path_check = Path::new(file_path.as_path());
            if path_check.exists() {
                if path_check.is_file(){
                    let file = FileRead::new("".to_string(),Some("/tmp/".to_string()));
                    println!("Watching file: {}",file_path.to_string_lossy());
                    file.watch()?
                } else if  path_check.is_dir(){
                    let folder = FileRead::new("".to_string(),Some("/tmp/".to_string()));
                    println!("Watching folder: {}",file_path.to_string_lossy());
                    folder.watch()?
                } else {
                    println!("'{}' exists but is neither a file nor a directory.", path_check.display());
                }
            } else {
                println!("This Path '{}' does not exist.",file_path.to_string_lossy());
            }
        }
    }


    // let file = FileRead::new("".to_string(),Some("/tmp/".to_string()));

    // file.watch()
    Ok(())

}
