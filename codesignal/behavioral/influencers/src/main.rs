mod influencer;

use influencer::{Influencer, MediaFollower, UserFollower};

fn main() {
    // TODO: Create an Influencer object
    let mut influencer = Influencer::new();
    // TODO: Create a UserFollower object named "Alice"
    let alice = UserFollower::new("Alice".to_string());
    // TODO: Create a MediaFollower object named "Life News"
    let life_news = MediaFollower::new("Life News".to_string());
    // TODO: Add both observers to the influencer's observers list
    influencer.add_observer(Box::new(alice));
    influencer.add_observer(Box::new(life_news));
    // TODO: Post an update "New post about travel!" and notify all observers
    influencer.post_update("New post about travel!");
    // TODO: Remove "Alice" from the influencer's observers list
    influencer.remove_observer(0);
    // TODO: Post another update "New post about fashion!" and notify the remaining observer
    influencer.post_update("New post about fashion!");
}
