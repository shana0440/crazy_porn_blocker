use egg_mode::user::TwitterUser;
use std::string::ToString;

pub struct User {
    user: TwitterUser,
}

impl ToString for User {
    fn to_string(&self) -> String {
        format!("{} @{}", &self.user.name, &self.user.screen_name)
    }
}

impl User {
    pub fn new(user: TwitterUser) -> User {
        User { user }
    }

    pub fn id(&self) -> u64 {
        self.user.id
    }
}
