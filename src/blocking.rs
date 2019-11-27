use futures03::{Future, TryFutureExt};
use tokio::task;
use warp::Rejection;

pub fn run<R>(
    f: impl FnOnce() -> Result<R, Rejection> + Send + 'static,
) -> impl Future<Output = Result<R, Rejection>>
where
    R: Send + 'static,
{
    task::spawn_blocking(f).unwrap_or_else(|e| Err(warp::reject::custom(e.to_string())))
}
