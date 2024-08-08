use std::env;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use notify::{Watcher, RecommendedWatcher,RecursiveMode, Result, EventKind};
use notify::event::Event;
use chrono::prelude::Local;

pub struct FileRead {
    pub path_base : String,
    pub name: String
}


fn get_datetime_local(action: String, event: Event)-> String{
    
    let local = Local::now();
    let formatted = format!("{}: {}: {:?}", local.format("%Y-%m-%d %H:%M:%S").to_string(), action, event.paths);

    formatted



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

    pub fn full_path(&self)-> PathBuf{
        // Return full path rute in PathBuf 
        Path::new(&self.path_base).join(&self.name)
    }

    
    pub fn str_full_path(&self)-> String{
        // Return full path rute in string format
        self.full_path().to_string_lossy().into_owned()
    }

    // pub fn path_buf(&self) -> PathBuf {
    //     self.full_path()
    // }

    pub fn watch(&self) -> Result<()>{
        // Create Channel  to recieve the events
        let (tx, rx) = channel();

        // Create a watcher object, delivering devounced evetns.
         // The duration is how long the watcher should wait after an event
         
         let mut watcher = RecommendedWatcher::new(tx, notify::Config::default())?;

         watcher.watch(&self.full_path(), RecursiveMode::NonRecursive)?;

        //  println!("Watching file: {:?}", self.full_path());
 
         // Process events
         loop {
            match rx.recv() {
                Ok(Ok(event)) => match event.kind {
                    // CHECK THIS: EVENT ACCESS (READ AND CLOSE ) DO NOT WORKING
                    // EventKind::Access(kind) => match kind {
                    //     AccessKind::Read => {
                    //         println!("File accessed (read): {:?}", event.paths);
                    //     },
                    //     AccessKind::Close(_) => {
                    //         println!("File was close (close): {:?}", event.paths);
                    //     },
                    //     _ =>{
                    //         ()
                    //     }
                    //     // println!("Access Read: {:?}", event.paths);
                    // },
                    EventKind::Create(_) => {
                        let str_out = get_datetime_local("Create".to_string(), event);
                        println!("{}", str_out);
                        // println!("Create: {:?}", event.paths);
                    },
                    EventKind::Modify(_) => {
                        let str_out = get_datetime_local("Modify".to_string(), event);
                        println!("{}", str_out);
                        // println!("Modify: {:?}", event.paths);
                    },
                    EventKind::Remove(_) => {
                        let str_out = get_datetime_local("Remove".to_string(), event);
                        println!("{}", str_out);
                        // println!("Remove: {:?}", event.paths);
                    },
                    // EventKind::Access(AccessKind::Close(_)) => {
                    //     println!("Close: {:?}", event.paths);
                    // },
                    _ => (),
                },
                Ok(Err(e)) => println!("watch error: {:?}", e),
                Err(e) => println!("recv error: {:?}", e),
            }
        }
     }
}


