use futures::future::Future;
use rdkafka::ClientConfig;
use rdkafka::error::KafkaError;
use rdkafka::message::OwnedMessage;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::boxed::Box;
use std::time::Duration;

async fn send(producer: &FutureProducer, topic: &str, message: &str)  {
    let ex = producer.send(
        FutureRecord::to(&topic)
            .payload(message.as_bytes())
            .key("alice"),
            Duration::from_secs(0),
    );
    ex.await;
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let topic = "test".to_owned();
    let value = "test".to_owned();
    
    let mut kafka_config = ClientConfig::new();
    kafka_config.set("bootstrap.servers", "az19d1-hestia.servicebus.windows.net:9093 ");
    kafka_config.set("security.protocol", "SASL_SSL");
    kafka_config.set("sasl.mechanisms", "PLAIN");
    kafka_config.set("sasl.username", "$ConnectionString");
    kafka_config.set("sasl.password", "Endpoint=sb://az19d1-hestia.servicebus.windows.net/;SharedAccessKeyName=base-manage;SharedAccessKey=iUtVFsGoyW3E5UDOMV0XCx/B7c5uLbonMxe+loornQQ=");
    
   
    let producer: &FutureProducer = kafka_config.create()?;

    send(&producer, &topic, &value);


    // let x = producer.send(
    //     FutureRecord::to(&topic)
    //         .payload(value.as_bytes())
    //         .key("alice"),
    //         Duration::from_secs(0),
    // );
   
    
    Ok(())
}