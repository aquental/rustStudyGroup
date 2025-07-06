mod bookstore;
mod subscriber;

use bookstore::BookStore;
use subscriber::ConcreteSubscriber;

fn main() {
    let mut store = BookStore::new();

    let subscriber1 = Box::new(ConcreteSubscriber::new("Subscriber 1"));
    let subscriber2 = Box::new(ConcreteSubscriber::new("Subscriber 2"));

    store.add_subscriber(subscriber1);
    store.add_subscriber(subscriber2);

    store.notify_new_arrival("New Book 1");
    store.remove_subscriber("Subscriber 1");
    store.notify_new_arrival("New Book 2");
}
