use std::{future::Future, sync::Arc};

use tokio::{
    runtime::{self, Handle, Runtime},
    sync::Mutex,
    task,
};

// requires setting up the .cargo/config.toml
pub async fn run_named_task<F>(task_name: String, runtime: Arc<Runtime>, fut: F)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    task::Builder::new()
        .name(&task_name)
        .spawn_on(fut, runtime.handle())
        .unwrap()
        .await;
}

pub fn setup_tokio_runtime() -> Runtime {
    runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("scan_runtime")
        .enable_io()
        .enable_time()
        .enable_metrics_poll_count_histogram()
        .build()
        .expect("Failed to build Tokio Runtime.")
}
