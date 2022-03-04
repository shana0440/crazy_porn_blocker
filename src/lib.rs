mod config;
pub mod user;

use crate::config::Config;
use crate::user::User;
use egg_mode::{auth, user::UserID, KeyPair, Token};

struct Auth {
    user: UserID,
    token: Token,
}

pub struct CrazyPornBlocker {
    config: Config,
    auth: Option<Auth>,
}

impl CrazyPornBlocker {
    pub fn new() -> CrazyPornBlocker {
        CrazyPornBlocker {
            config: Config::load(),
            auth: None,
        }
    }

    pub async fn authenticate_url(&self) -> (String, KeyPair, KeyPair) {
        println!("consumer_api_key: {}", self.config.con_token().key);
        println!(
            "consumer_api_key_secret: {}",
            self.config.con_token().secret
        );
        let con_token = self.config.con_token();
        let request_token = egg_mode::auth::request_token(&con_token, "oob")
            .await
            .unwrap();
        let auth_url = egg_mode::auth::authenticate_url(&request_token);
        (auth_url, con_token, request_token)
    }

    pub async fn access_token(&mut self, pin: String, con_token: KeyPair, request_token: KeyPair) {
        let (token, user_id, _screen_name) = auth::access_token(con_token, &request_token, pin)
            .await
            .unwrap();
        self.auth = Some(Auth {
            user: user_id.into(),
            token,
        });
    }

    pub async fn followers(&self) -> Vec<User> {
        let auth = self.auth.as_ref().expect("Please login first");

        let mut list =
            egg_mode::user::followers_of(auth.user.clone(), &auth.token).with_page_size(20);
        let mut users = vec![];
        loop {
            let resp = list.call().await.unwrap();
            for user in resp.response.users {
                users.push(User::new(user));
            }
            if resp.response.next_cursor == 0 {
                break;
            }
            list.next_cursor = resp.response.next_cursor;
        }
        users
    }

    pub async fn block(&self, users: Vec<&User>) {
        let auth = self.auth.as_ref().expect("Please login first");
        for user in users {
            egg_mode::user::block(user.id(), &auth.token).await.unwrap();
        }
    }
}
