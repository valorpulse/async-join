use trpl;

fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let vals = vec! [
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("other"),
            String::from("side"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(std::time::Duration::from_millis(500)).await;
        }

        while let Some(val) = rx.recv().await {
            println!("got: {}", val);
        }
    })    
}
