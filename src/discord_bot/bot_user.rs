use serenity::all::{CacheHttp, GuildId, User};

#[derive(Debug)]
pub struct BotUser {
    pub display_name: String,
    pub nickname: Option<String>,
    pub id: u64,
}

impl From<User> for BotUser {
    fn from(value: User) -> Self {
        Self {
            display_name: value.display_name().to_string(),
            nickname: None,
            id: value.id.get(),
        }
    }
}
impl From<&User> for BotUser {
    fn from(value: &User) -> Self {
        Self {
            display_name: value.display_name().to_string(),
            nickname: None,
            id: value.id.get(),
        }
    }
}

impl From<u64> for BotUser {
    fn from(value: u64) -> Self {
        Self {
            display_name: String::new(),
            nickname: None,
            id: value,
        }
    }
}

impl BotUser {
    pub async fn from_user<T>(user: &User, http: T, guild_id: Option<GuildId>) -> Self
    where
        T: CacheHttp,
    {
        Self {
            display_name: user.display_name().into(),
            nickname: match guild_id {
                Some(g) => user.nick_in(http, g).await,
                _ => None,
            },
            id: user.id.get(),
        }
    }
}
