use dotenv::dotenv;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use tokio::time::sleep;

use crate::{get_addr, postgres, router::create_routes};

pub struct Server {
    pub started: AtomicBool,
}

impl Server {
    pub fn new() -> Server {
        Server {
            started: AtomicBool::new(false),
        }
    }

    pub async fn init(&mut self) {
        if !self.started.load(Ordering::Relaxed) {
            thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().expect("runtime starts");
                rt.spawn(run());
                loop {
                    thread::sleep(Duration::from_millis(100_000));
                }
            });
            sleep(Duration::from_millis(100)).await;
            self.started.store(true, Ordering::Relaxed);
        }
    }
}

async fn run() {
    dotenv().ok();
    pretty_env_logger::init();

    let pg_pool = postgres::init()
        .await
        .expect("Unable to initialize Postgres Pool.");
    let router = create_routes(pg_pool);
    let addr = get_addr();

    warp::serve(router).run(addr).await;
}
