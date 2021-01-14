#[macro_use]
extern crate quick_error;

use quick_error::ResultExt;
use std::fs::File;
use std::fs::OpenOptions;
use std::{ io, result };

const MAX_DOCS_CREATED_PER_MINUTE: u8 = 10;

fn num_documents_created_in_last_minute() -> u8 {
    2
}

quick_error! {
    #[derive(Debug)]
    pub enum DocumentServiceError {
        RateLimitExceeded {
            display("You have exceeded the allowed number of documents per minute.")
        }
        Io(filename: String, cause: io::Error) {
            display("I/O error: {} for file name {}", cause, filename)
            context(filename: &'a str, cause: io::Error)
                -> (filename.to_string(), cause)
        }
    }
}

pub type Result<T> = result::Result<T, DocumentServiceError>;

pub fn create_document(filename: &str) -> Result<File> {
    if num_documents_created_in_last_minute() > MAX_DOCS_CREATED_PER_MINUTE {
        return Err(DocumentServiceError::RateLimitExceeded);
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)
        .context(filename)?;

    Ok(file)
}

fn create_project(project_name: &str) -> Result<()> {
    create_document(&format!("{}-draft1", project_name))?;
    create_document(&format!("{}-draft2", project_name))?;
    create_document(&format!("{}-revision1", project_name))?;
    create_document(&format!("{}-revision2", project_name))?;

    Ok(())
}


fn main() {
    match create_project("my-project") {
        Ok(()) => println!("Project created!"),
        Err(e) => println!("Project creation failed: {}", e),
    }
}
