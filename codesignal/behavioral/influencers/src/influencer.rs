use std::collections::HashMap;

// TODO: Define the Observer trait with an update method
// - The update method should accept a string argument as a post

// TODO: Implement the Influencer struct with methods to manage observers
// - Use a HashMap to store observers with a unique id
// - Implement the add_observer method to add an observer to the list
// - Implement the remove_observer method to remove an observer using the id
// - Implement the post_update method to notify all observers about the new post

// TODO: Implement the UserFollower struct that inherits from Observer
// - The constructor should initialize the name of the user follower
// - The update method should print "<name> (User) saw post: <post>"

// TODO: Implement the MediaFollower struct that inherits from Observer
// - The constructor should initialize the outlet name of the media follower
// - The update method should print "<outletName> (Media) reported: <post>"
