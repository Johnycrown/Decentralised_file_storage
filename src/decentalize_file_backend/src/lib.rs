
use ic_cdk::export;
use ic_cdk::export::candid::{CandidType, Deserialize, Principal};
use serde::Serialize;


// Define data structures for files and directories
#[derive(Clone, CandidType, Deserialize, Serialize)]
struct File {
    name: String,
    content: Vec<u8>,
}

#[derive(Clone, CandidType, Deserialize, Serialize)]
struct Directory {
    name: String,
    files: Vec<File>,
    subdirectories: Vec<Directory>,
}

// Define a trait for interacting with the file storage system
#[export]
trait FileStorage {
    fn upload_file(&mut self, directory_path: String, file_name: String, content: Vec<u8>) -> Result<(), String>;
    fn download_file(&self, file_path: String) -> Result<Vec<u8>, String>;
    fn create_directory(&mut self, parent_directory_path: String, directory_name: String) -> Result<(), String>;
}

// Implement the FileStorage trait for a decentralized file storage canister
struct DecentralizedFileStorage {
    root_directory: Directory,
}

impl FileStorage for DecentralizedFileStorage {
    fn upload_file(&mut self, directory_path: String, file_name: String, content: Vec<u8>) -> Result<(), String> {
        // Find the directory specified by directory_path
        // If directory doesn't exist, return an error
        
        // Create a new File instance with the provided content
        let new_file = File {
            name: file_name.clone(),
            content,
        };

        // Add the new file to the directory's files vector
        // Return success
        self.root_directory.files.push(new_file);
        Ok(())
    }

    fn download_file(&self, file_path: String) -> Result<Vec<u8>, String> {
        // Find the file specified by file_path
        // If file doesn't exist, return an error
        
        // Return the content of the file
        for file in &self.root_directory.files {
            if file.name == file_path {
                return Ok(file.content.clone());
            }
        }
        Err("File not found".to_string())
    }

    fn create_directory(&mut self, parent_directory_path: String, directory_name: String) -> Result<(), String> {
        // Find the parent directory specified by parent_directory_path
        // If parent directory doesn't exist, return an error
        
        // Create a new Directory instance
        let new_directory = Directory {
            name: directory_name.clone(),
            files: Vec::new(),
            subdirectories: Vec::new(),
        };

        // Add the new directory to the parent directory's subdirectories vector
        // Return success
        self.root_directory.subdirectories.push(new_directory);
        Ok(())
    }
}

#[export_name = "canister_query upload_file"]
fn upload_file(directory_path: String, file_name: String, content: Vec<u8>) -> Result<(), String> {
    let mut storage = DecentralizedFileStorage {
        root_directory: Directory {
            name: "root".to_string(),
            files: Vec::new(),
            subdirectories: Vec::new(),
        },
    };
    storage.upload_file(directory_path, file_name, content)
}

#[export_name = "canister_query download_file"]
fn download_file(file_path: String) -> Result<Vec<u8>, String> {
    let storage = DecentralizedFileStorage {
        root_directory: Directory {
            name: "root".to_string(),
            files: Vec::new(),
            subdirectories: Vec::new(),
        },
    };
    storage.download_file(file_path)
}

#[export_name = "canister_query create_directory"]
fn create_directory(parent_directory_path: String, directory_name: String) -> Result<(), String> {
    let mut storage = DecentralizedFileStorage {
        root_directory: Directory {
            name: "root".to_string(),
            files: Vec::new(),
            subdirectories: Vec::new(),
        },
    };
    storage.create_directory(parent_directory_path, directory_name)
}
