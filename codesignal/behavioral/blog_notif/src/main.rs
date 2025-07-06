mod blog_publisher;
mod concrete_subscriber;

use blog_publisher::BlogPublisher;
use concrete_subscriber::ConcreteSubscriber;

fn main() {
    let mut blog_publisher = BlogPublisher::new();

    let subscriber1 = Box::new(ConcreteSubscriber::new("Subscriber 1"));
    let subscriber2 = Box::new(ConcreteSubscriber::new("Subscriber 2"));

    blog_publisher.add_subscriber("sub1_id".to_string(), subscriber1);
    blog_publisher.add_subscriber("sub2_id".to_string(), subscriber2);

    blog_publisher.publish("New Article 1");

    blog_publisher.remove_subscriber("sub1_id");
    blog_publisher.publish("New Article 2");
}
