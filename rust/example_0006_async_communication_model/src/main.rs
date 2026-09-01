mod broadcast_test;
mod mpsc_test;
mod oneshot_test;

use broadcast_test::run_broadcast_case;
use mpsc_test::{run_mpsc_case, run_mpsc_case_2};
use oneshot_test::run_oneshot_case;

#[tokio::main]
async fn main() {
    run_mpsc_case().await;
    run_mpsc_case_2().await;
    run_oneshot_case().await;
    run_broadcast_case().await;
}
