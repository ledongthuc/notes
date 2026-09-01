use std::time::Duration;
use tokio::sync::mpsc;

pub async fn run_mpsc_case() {
    println!("*** Test mpsc (multi-producers - single consumer) ***");

    let (tx, mut rx) = mpsc::channel::<i32>(32);
    let tx2 = tx.clone();
    let tx3 = tx.clone();

    tokio::spawn(async move {
        for i in 1..4 {
            let random_secs = rand::random_range(1..=3);
            tokio::time::sleep(Duration::from_secs(random_secs)).await;
            println!("Producer 1: send {i}");
            tx.send(i).await.unwrap();
        }
    });

    tokio::spawn(async move {
        for i in 1..4 {
            let random_secs = rand::random_range(1..=3);
            tokio::time::sleep(Duration::from_secs(random_secs)).await;
            let data = i * 10;
            println!("Producer 2: send {data}");
            tx2.send(data).await.unwrap();
        }
    });

    tokio::spawn(async move {
        for i in 1..4 {
            let random_secs = rand::random_range(1..=3);
            tokio::time::sleep(Duration::from_secs(random_secs)).await;
            let data = i * 100;
            println!("Producer 3: send {data}");
            tx3.send(data).await.unwrap();
        }
    });

    while let Some(n) = rx.recv().await {
        println!("Consumer got {n}")
    }

    println!("*** DONE ***")
}

struct Quote {
    source: String,
    value: f64,
    ticket: String,
}

pub async fn run_mpsc_case_2() {
    println!("*** Test mpsc2: Price Quote Update ***");

    let (tx_bloomberg, mut rx) = mpsc::channel::<Quote>(64);
    let tx_router = tx_bloomberg.clone();

    tokio::spawn(async move {
        tx_bloomberg
            .send(Quote {
                source: String::from("Bloomberg"),
                ticket: String::from("APPL"),
                value: 10.5,
            })
            .await
            .unwrap();
    });

    tokio::spawn(async move {
        tx_router
            .send(Quote {
                source: String::from("Reuter"),
                ticket: String::from("APPL"),
                value: 10.4,
            })
            .await
            .unwrap();
    });

    while let Some(quote) = rx.recv().await {
        let ticket = quote.ticket;
        let source = quote.source;
        let value = quote.value;
        println!("Process [{source}] {ticket}: {value}");
    }

    println!("*** DONE ***")
}
