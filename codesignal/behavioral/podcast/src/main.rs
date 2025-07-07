mod listener;
mod podcast_channel;

use listener::ConcreteListener;
use podcast_channel::PodcastChannel;

fn main() {
    // TODO: Create an instance of PodcastChannel
    let mut podcast_channel = PodcastChannel::new();

    // TODO: Create instances of listeners with the names:
    // - "Listener 1"
    // - "Listener 2"
    let listener1 = ConcreteListener::new("Listener 1");
    let listener2 = ConcreteListener::new("Listener 2");

    // TODO: Add listeners to the podcast channel
    podcast_channel.add_listener("Listener 1", Box::new(listener1));
    podcast_channel.add_listener("Listener 2", Box::new(listener2));

    // TODO: Release an episode "Episode 1: Understanding Rust"
    podcast_channel.release_episode("Episode 1: Understanding Rust");

    // TODO: Remove the first listener from the podcast channel
    podcast_channel.remove_listener("Listener 1");

    // TODO: Release another episode "Episode 2: Advanced Rust Concepts"
    podcast_channel.release_episode("Episode 2: Advanced Rust Concepts");
}
