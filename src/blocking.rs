use tokio::task;
use warp::Rejection;

pub async fn run<R>(
    f: impl FnOnce() -> Result<R, Rejection> + Send + 'static,
) -> Result<R, Rejection>
where
    R: Send + 'static,
{
    task::spawn_blocking(f).await.unwrap()
}
