use std::{thread, time};
use cot_publisher::CotPublisher;

fn main() {
    eprintln!("Hello, TAK!");

    let mut publisher = CotPublisher::new(
        "test-uid-1234",
        "a-f-G-U-C",
        // Some("239.2.3.1:6969"),
        None,
        Some(("127.0.0.1", 8088))
    );

    let now = std::time::SystemTime::now();
    let since_the_epoch = now
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");
    eprintln!("{}", since_the_epoch.as_millis() as u64);

    eprintln!("Publishing");
    publisher.connect();
    (0..10).for_each(|i| {
        publisher.publish();
        thread::sleep(time::Duration::from_secs(1));
        eprintln!("{i}");
    });
    eprintln!("Done. Goodbye.");
}
