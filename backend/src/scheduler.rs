use fang::AsyncQueue;
use fang::AsyncWorkerPool;

#[derive(Debug)]
pub struct Scheduler {}

impl Scheduler {
    pub async fn init(database_url: String) -> AsyncQueue {
        tracing::info!("Initializing Scheduler...");
        let max_pool_size: u32 = 3;
        let mut queue = AsyncQueue::builder()
            .uri(database_url)
            .max_pool_size(max_pool_size)
            .build();

        queue.connect().await.unwrap();
        tracing::info!(" - Queue connected...");

        let mut pool: AsyncWorkerPool<AsyncQueue> = AsyncWorkerPool::builder()
            .number_of_workers(10_u32)
            .queue(queue.clone())
            .build();

        tracing::info!(" - Pool created...");

        pool.start().await;
        tracing::info!(" - Workers started.");

        queue
    }
}
