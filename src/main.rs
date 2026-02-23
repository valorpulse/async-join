use trpl;
use std::time::Duration;
use std::pin::Pin;
use std::future::Future;

fn main() {
    trpl::block_on(async {
         let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };
        let futures = vec![
            Box::pin(tx1_fut) as Pin<Box<dyn Future<Output = ()>>>,
            Box::pin(rx_fut) as Pin<Box<dyn Future<Output = ()>>>,
            Box::pin(tx_fut) as Pin<Box<dyn Future<Output = ()>>>,
        ];

        trpl::join_all(futures).await;
    })    
}
