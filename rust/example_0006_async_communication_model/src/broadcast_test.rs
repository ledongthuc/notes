use tokio::sync::broadcast;

pub async fn run_broadcast_case() {
    println!("*** Test Broadcast ***");

    let (tx1, mut rx1) = broadcast::channel::<String>(64);
    let tx2 = tx1.clone();
    let mut rx2 = tx1.subscribe();

    tokio::spawn(async move {
        println!("-Start game client 1");
        tokio::spawn(async move {
            for _ in 0..=2 {
                tx1.send(String::from("Client 1: shoot")).unwrap();
            }
        });

        while let Ok(n) = rx1.recv().await {
            println!("Client 1 render: {n}")
        }
    });

    tokio::spawn(async move {
        println!("-Start game client 2");
        tokio::spawn(async move {
            for _ in 0..=4 {
                tx2.send(String::from("Client 2: shoot")).unwrap();
            }
        });
    });

    while let Ok(n) = rx2.recv().await {
        println!("Client 2 render: {n}")
    }

    println!("*** DONE ***")
}
