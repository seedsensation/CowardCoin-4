use super::Serialisable;
pub struct User {
    user_id: i32,
    coin_count: i32,
}

impl User {
    /// Get the username of the user
    ///
    /// This should be replaced later on with a function to get the username from Discord.
    pub fn get_username(&self) -> String {
        "username".into()
    }
}

impl Serialisable for User {
    fn get_id(&self) -> i32 {
        self.user_id
    }

    fn serialise(&self) -> String {
        todo!()
    }

    fn deserialise(to_deserialise: String) -> Self {
        todo!()
    }
}
