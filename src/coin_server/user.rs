use super::Serialisable;
#[derive(Clone)]
pub struct User {
    pub user_id: i32,
    pub coin_count: i32,
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
    const PART_COUNT: usize = 2;

    fn get_id(&self) -> i32 {
        self.user_id
    }

    fn serialise(&self) -> String {
        let mut result: String = self.get_id().to_string();
        result.push_str(";");
        result.push_str(self.coin_count.to_string().as_str());
        result
    }

    fn deserialise(to_deserialise: String) -> Self {
        let parts = Self::split_string(&to_deserialise);

        // length of parts is PART_COUNT or less
        // so we can pull them out with get(),
        // and have default values for anything that isn't a user ID.
        // this means we can add new values to User without corrupting existing data.
        Self {
            // user_id
            // no default value
            user_id: parts
                .get(0)
                .unwrap()
                .parse::<i32>()
                .expect("Invalid User ID"),
            // coin_count
            // default value: 0
            coin_count: parts
                .get(1)
                .unwrap_or(&"0")
                .parse::<i32>()
                .expect("Invalid Coin Count"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialise() {
        let to_deserialise: String = "1;2".into();
        let deserialised = User::deserialise(to_deserialise.clone());
        let reserialised = deserialised.serialise();
        println!("{}", to_deserialise);
        println!("{}", reserialised);
        assert!(to_deserialise == reserialised, "Serialisation failed");
        assert!(User::deserialise("1".into()).serialise() == "1;0");
    }
}
