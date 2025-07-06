mod celebrity;
mod follower;

use celebrity::Celebrity;
use follower::ConcreteFollower;

fn main() {
    let mut celebrity = Celebrity::new();

    let follower1 = Box::new(ConcreteFollower::new("Follower 1"));
    let follower2 = Box::new(ConcreteFollower::new("Follower 2"));

    celebrity.add_follower("follower1_id".to_string(), follower1);
    celebrity.add_follower("follower2_id".to_string(), follower2);

    celebrity.post_update("New Album Release!");

    celebrity.remove_follower("follower1_id");
    celebrity.post_update("Concert Dates Announced!");
}
