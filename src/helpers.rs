use std::time::SystemTime;

use rand::prelude::*;

pub fn random_between(low: i64, high: i64) -> i64 {
    (low..high)
        .collect::<Vec<i64>>()
        .choose(&mut rand::rng())
        .unwrap()
        .clone()
}

#[inline]
pub fn random_from<'a, T>(vals: &'a Vec<T>) -> &'a T {
    vals.choose(&mut rand::rng()).unwrap()
}

#[inline]
pub fn random_from_owned<T>(vals: &Vec<T>) -> T
where
    T: Clone,
{
    (vals.choose(&mut rand::rng()).unwrap()).clone()
}

#[inline]
pub fn s_if(val: i64) -> String {
    match val {
        1 => "",
        _ => "s",
    }
    .into()
}

#[macro_export]
macro_rules! choose_message {
    () => {
	"`_`".to_string()
    };
    ( $( $x:expr),*$(,)*) => {
        crate::helpers::random_from(&vec![$(crate::helpers::MessageType::from($x),)*]).format()
    };
}

/// Get multiple mutable users from their ID.
///
/// SAFETY: Will always check whether the index is valid before dereferencing.
/// Be careful, though - it is assumed that a recipient exists.
#[macro_export]
macro_rules! get_mut_users_from_ids {
    ($($user:ident),+ in $server:ident) => {
	unsafe {
	    ($({
		match $server.users.binary_search_by_key(&$user.id, |x| x.id) {
		    Ok(v) => Some(&mut *($server.users.as_mut_ptr().add(v))),
		    Err(_) => None
		}
	    }),+)
	}
    };
}

#[macro_export]
macro_rules! get_index_from_id {
    ($user:ident in $server:ident) => {{
        match $server.users.binary_search_by_key(&$user.id, |x| x.id) {
            Ok(v) => v,
            Err(_) => {
                $server.users.push(crate::coin_server::CoinUser::new(
                    $user.id,
                    $user.nickname.clone(),
                    $user.display_name.clone(),
                ));
                ($server.users.len() - 1)
            }
        }
    }};
}

#[inline]
pub fn default_timestamp() -> SystemTime {
    SystemTime::UNIX_EPOCH
}

pub fn seconds_to_string(full_time: i64) -> String {
    let seconds = full_time % 60;
    let minutes = (full_time / 60) % 60;
    let hours = full_time / 60 / 60;
    format!(
        "{}{}{}",
        if hours > 0 {
            format!("{hours} hour{}, ", s_if(hours))
        } else {
            format!("")
        },
        if minutes > 0 {
            format!("{minutes} minute{}, ", s_if(minutes))
        } else {
            format!("")
        },
        format!(
            "{}{seconds} second{}",
            if hours > 0 || minutes > 0 { "and " } else { "" },
            s_if(seconds)
        )
    )
}

#[derive(Clone)]
pub enum MessageType {
    S(&'static str),
    O(String),
}
impl From<&'static str> for MessageType {
    #[inline]
    fn from(value: &'static str) -> Self {
        Self::S(value)
    }
}

impl From<String> for MessageType {
    #[inline]
    fn from(value: String) -> Self {
        Self::O(value)
    }
}

impl MessageType {
    #[inline]
    pub fn format(&self) -> String {
        match self {
            Self::O(val) => val.to_string(),
            Self::S(val) => String::from(*val),
        }
    }
}
