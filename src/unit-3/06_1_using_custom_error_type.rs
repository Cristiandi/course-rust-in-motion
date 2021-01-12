use std::error::Error;
use std::fmt;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;

const MAX_DOCS_CREATED_PER_MINUTE: u8 = 10;

fn num_documents_created_in_last_minute() -> u8 {
    2
}

#[derive(Debug)]
pub enum DocumentServiceError {
    RateLimitExceeded,
    Io(io::Error),
}

// aquí se esta implementando el rasgo Error
// la funcion description no es necesaria en versiones superiores de rust 1.27
// el rasgo error requiere  que se implementen los rasgos Debug y Display por eso el #[derive(Debug)]
impl Error for DocumentServiceError {
    fn description(&self) -> &str {
        use DocumentServiceError::*;
        match *self {
            RateLimitExceeded => "rate limit exceeded",
            Io(_) => "I/O error",
        }
    }
}

// aquí emplemento algo así como funa función formateadora, así cuando sucede un error, se identifica el tipo
// y basado en el tipo del error se escribe y se formatea el mensaje
impl fmt::Display for DocumentServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use DocumentServiceError::*;
        match *self {
            RateLimitExceeded => write!(
                f,
                "You have exceeded the allowed number of documents per minute."
            ),
            Io(ref io) => write!(f, "I/O error: {}", io),
        }
    }
}

// aquí se esta implementanto el rasgo From
// el cual sirve para  convertir un tipo de error en otro tipo de error
// en este caso lo utilizo porque cuando tenga el error tipo io::Error yo lo quiero convertir en un error 
// DocumentServiceError
impl From<io::Error> for DocumentServiceError {
    fn from(other: io::Error) -> Self {
        DocumentServiceError::Io(other)
    }
}

// implemento este result type alias para evitar el DocumentServiceError como error type en todas las funciones
// donde lo utilice
use std::result;
pub type Result<T> = result::Result<T, DocumentServiceError>;

pub fn create_document(filename: &str) -> Result<File> {
    if num_documents_created_in_last_minute() > MAX_DOCS_CREATED_PER_MINUTE {
        return Err(DocumentServiceError::RateLimitExceeded);
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(filename)?;

    Ok(file)
}

fn main() {
    let file = create_document("test.txt").expect("something went wrong!");

    let metadata = file.metadata().expect("error getting meta data.");

    println!("metadata: {:?}", metadata);

}
