# File Watcher

File Watcher is a Rust application for monitoring files and directories using the `notify` crate. This application allows you to watch for changes such as creation, modification, and deletion of files in real-time.
** This repository is intended for learning purposes. It demonstrates how to use the notify crate to monitor filesystem events in Rust.**
## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Installation

Clone the repository and navigate to the project directory:

```sh
$ git clone <REPOSITORY_URL>
$ cd <PROJECT_DIRECTORY>
```
## Usage
## Monitor the Current Directory
If no arguments are passed, the application will monitor the directory where the program is executed:

```sh
$ cargo run
```

## Monitor Specific Files or Directories
You can pass file or directory paths as arguments to monitor them:

```sh 
$ cargo run -- <file_or_directory_path> 
```
##  Example
To monitor a specific file:
```sh 
$ cargo run -- /path/to/your/file.txt
```

To monitor a specific directory:

```sh 
$ cargo run -- /path/to/your/directory  path/to/your/directory2
```

## Functionality
The application uses the notify crate to watch for filesystem events. The monitored events include:

- File creation
- File modification
- File deletion


## Code Structure

FileRead
A struct that represents the configuration for monitoring a file or directory.

Methods
- **new(name: String, path_base: Option<PathBuf>) -> Self**: Constructor to create a new **FileRead** instance.
- **full_path(&self) -> PathBuf**: Returns the full path of the file or directory.
- **watch(&self) -> Result<()>**: Starts watching the file or directory.

### `main`
The main function processes command-line arguments and sets up file or directory watching.

## Learning Repository
This repository is intended for learning purposes. It demonstrates how to use the notify crate to monitor filesystem events in Rust.

