use std::env;

use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;

#[tokio::main]
async fn main() {
    // Define Kafka producer configuration
    let producer: FutureProducer = rdkafka::config::ClientConfig::new()
        .set(
            "bootstrap.servers",
            env::var("KAFKA_BOOTSTRAP_ADDRESS").unwrap(),
        )
        .create()
        .expect("Producer creation error");

    // Define the Kafka topic to produce messages to
    let kafka_topic = &env::var("KAFKA_TOPIC").unwrap();

    // Sample data to be written to Kafka
    let sample_data = "Hello, Kunjesh!";

    // Produce a message to the Kafka topic
    let record = FutureRecord::to(kafka_topic)
        .key("kunjesh_key")
        .payload(sample_data);

    // Use `Timeout::Never` to retry sending forver incase of failure
    let timeout = Timeout::Never;

    match producer.send(record, timeout).await {
        Ok((partition, offset)) => {
            println!(
                "Message sent successfully: partition={}, offset={}",
                partition, offset
            )
        }
        Err((err, _)) => eprintln!("Error sending message: {:?}", err),
    }
}
