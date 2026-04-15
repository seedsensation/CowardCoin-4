use super::Request;
use super::User;
use super::file_management::{load_from_file, save_to_file};
use std::sync::mpsc::Receiver;
use tokio::time::{self, Duration, Interval};

pub struct Server {
    pub coin_available: bool,
    pub users: Vec<User>,
    pub receiver: Receiver<Request>,
    check_interval: Interval,
    coin_interval: Interval,
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
            receiver: receiver,
            check_interval: time::interval(Duration::from_millis(100)),
            coin_interval: time::interval(Duration::from_secs(1)),

            users: match load_from_file(Self::USER_FILENAME) {
                Ok(users) => users,
                Err(e) => panic!("Error loading file: {e:?}"),
            },
        }
    }

    pub async fn run() {}
}
