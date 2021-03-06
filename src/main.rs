use std::fmt::Display;

use rand::prelude::*;

fn one_in(n: u32) -> bool {
    thread_rng().gen_ratio(1, n)
}

/// Represents the state of a file. Either `Open` or `Closed`
#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

/// Struct for representing a "file"
#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
    pub state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    pub fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state == FileState::Closed {
            return Err(String::from("File is not open for reading"));
        }

        let mut tmp = self.data.clone();

        // Keep track of how much we read
        let read_length = tmp.len();

        // Make sure that our save buffer is sized appropriately to avoid
        // resizing as we read and append the temp buffer to it.
        save_to.reserve(read_length);
        save_to.append(&mut tmp);

        Ok(read_length)
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let mut f3 = File::new_with_data("3.txt", &vec![114, 117, 115, 116, 33]);
    let mut buffer: Vec<u8> = vec![];

    f3 = open(f3).unwrap();

    let f3_length = f3.read(&mut buffer).unwrap();

    f3 = close(f3).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{}", f3);
    println!("{} is {} bytes long", &f3.name, f3_length);
    println!("{}", text)
}
