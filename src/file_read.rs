use std::env;
use std::path::{Path, PathBuf};
pub struct FileRead {
    pub path_base : String,
    pub name: String
}

impl FileRead{
    pub fn new(name:String,path_base: Option<String>) -> Self {
        // If is None path_base use current dir file
        let path  = match path_base {
            Some(p) => p,
            None => env::current_dir()
                    .expect("Failed to get current directory")
                    .to_str()
                    .expect("Failed to convert path to string").to_string(),
        };

        FileRead {
            path_base: path,
            name,
        }
    }

    pub fn full_path(self)-> PathBuf{
        // Return full path rute in PathBuf 
        Path::new(&self.path_base).join(&self.name)
    }

    pub fn str_full_path(self)-> String{
        // Return full path rute in string format
        self.full_path().to_string_lossy().into_owned()
    }
}
