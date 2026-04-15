use super::Request;
use super::User;
use super::file_management::{load_from_file, save_to_file};
use std::io;
use std::io::Result;
use std::sync::mpsc::Receiver;

pub struct Server {
    pub coin_available: bool,
    pub users: Vec<User>,
    pub receiver: Receiver<Request>,
}

impl Server {
    const USER_FILENAME: &str = "./users.csv";
    pub fn save(&self) {
        match save_to_file(Self::USER_FILENAME, &self.users) {
            Ok(_) => println!("Users saved to file successfully!"),
            Err(e) => panic!("Error saving to user file: {e:?}"),
        }
    }

    pub fn load(receiver: Receiver<Request>) -> Self {
        Self {
            coin_available: false,
            users: match load_from_file(Self::USER_FILENAME) {
                Ok(users) => users,
                Err(e) => panic!("Error loading file: {e:?}"),
            },
            receiver: receiver,
        }
    }
}
