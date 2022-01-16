

use std::time::Duration;


fn print_1_10() -> impl Future<Item = (), Error = ()> {
    future::loop_fn(0, |i| {
        tokio_timer::sleep(Duration::from(_))
    })
}