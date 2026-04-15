use super::Server;
use std::fs::File;
use std::io::{self, BufRead, ErrorKind, Write};
use std::path::Path;

pub trait Serialisable {
    const PART_COUNT: usize;
    const DELIMITER: &str = ";";

    /// Gets the user's ID.
    fn get_id(&self) -> i32;
    /// Serialise an object into a string.
    fn serialise(&self) -> String;
    /// Deserialise a string into an object.
    fn deserialise(to_deserialise: String) -> Self;

    /// Splits a string into a vector of string literals, based on its `PART_COUNT`.
    ///
    /// Panics if its size is not equal to `PART_COUNT`.
    fn split_string(to_deserialise: &String) -> Vec<&str> {
        let parts: Vec<&str> = to_deserialise.split(Self::DELIMITER).collect();
        assert!(
            parts.len() <= Self::PART_COUNT,
            "Invalid data to be deserialised - please check your file format!"
        );
        return parts;
    }
}

/// Load a vector of a type to a given file
pub fn load_from_file<T>(filename: &str) -> io::Result<Vec<T>>
where
    T: Serialisable,
{
    let mut objects: Vec<T> = vec![];
    let lines = match read_lines(filename) {
        Ok(l) => Ok(l),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                File::create(filename);
                read_lines(filename)
            }
            _ => Err(error),
        },
    }?;

    for line in lines.map_while(Result::ok) {
        objects.push(T::deserialise(line));
    }
    Ok(objects)
}

/// Save a vector of a type to a given file
pub fn save_to_file<T>(filename: &str, items: &Vec<T>) -> io::Result<()>
where
    T: Serialisable,
{
    let file_result = File::open(filename);
    let mut file = match file_result {
        Ok(f) => Ok(f),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => File::create(filename),
            _ => Err(error),
        },
    }?;
    let mut output: String = String::new();
    for item in items {
        output.push_str(&item.serialise());
        output.push_str("\n");
    }
    file.write_all(output.as_bytes())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
