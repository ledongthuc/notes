use tokio::sync::oneshot;

pub async fn run_oneshot_case() {
    println!("*** Test oneshot ***");

    let (tx, rx) = oneshot::channel::<u64>();

    tokio::spawn(async move {
        println!("*** Load data from database ***");
        let _ = tx.send(12);
    });

    match rx.await {
        Ok(value) => println!("Loaded data: {value}"),
        Err(err) => println!("the worker dropped the sender without sending: {err}"),
    }

    println!("*** DONE ***")
}
