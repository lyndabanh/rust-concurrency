use tokio::sync::{Semaphore, mpsc};
use std::{sync::Arc, time::Duration};

#[tokio::main]
async fn main() {
    let num_items = 10;
    let concurrency_limit = 1;
    let semaphore = Arc::new(Semaphore::new(concurrency_limit));
    let (tx_errors, mut rx_errors) = mpsc::channel(100);

    for item in 0..num_items {
        let tx_errors_clone = tx_errors.clone();
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(1)).await;
            if item == 7 {
                let _ = tx_errors_clone.send(item).await;
            }
            drop(permit);
        });
        println!("item: {}", item);
    }
    while let Some(item) = rx_errors.recv().await {
        println!("Received item {} (error)", item);
    }
    // let _ = semaphore.acquire_many(concurrency_limit as u32).await.unwrap();
    println!("End of main");
}

// if item (for ex) 31 returns an err, transmit the info for the loop to stop
// use the tx to send anything (like empty object () ) if there's an error

// select block (a tokio macro)
    // you can give the select multiple diff futures. the select will run the first one that resolves
    // receiving from a channel is a future
    // if you get a message on that channel, you can break out of the loop (it interrupts the loop). Use break statement when you get this

// select, check for 2 conditions:
// 1: That we get a permit
// 2: That we receive a message on the channel that one of my tasks has errored

// select! {
//     rx_errors.recv() => {
//         println!("error received")
//     }
//     semaphore.acquire_many(num_items as u32) => {
//         println!("permit acquired")
//     }
// }
