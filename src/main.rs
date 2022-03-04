use crazy_porn_blocker::user::User;
use crazy_porn_blocker::CrazyPornBlocker;
use dialoguer::MultiSelect;

#[tokio::main]
async fn main() {
    let mut blocker = CrazyPornBlocker::new();
    let (url, con_token, request_token) = blocker.authenticate_url().await;
    println!("authenticate url: {}", url);
    println!("enter your pin code");
    let mut pin = String::new();
    std::io::stdin().read_line(&mut pin).unwrap();
    blocker.access_token(pin, con_token, request_token).await;
    let followers = blocker.followers().await;
    let chosen = MultiSelect::new().items(&followers).interact().unwrap();
    let mut chosen_followers = Vec::with_capacity(chosen.len());
    for item in chosen {
        chosen_followers.push(&followers[item]);
    }
    blocker.block(chosen_followers).await;
}
