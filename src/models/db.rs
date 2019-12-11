use extension_trait::extension_trait;
use warp::reject::{Reject, Rejection};

#[derive(Debug)]
struct DbError(diesel::result::Error);

impl Reject for DbError {}

#[extension_trait(pub)]
impl<T> DbErrorExt for Result<T, diesel::result::Error> {
    type Error = T;
    fn into_rejection(self) -> Result<Self::Error, Rejection> {
        self.map_err(|e| warp::reject::custom(DbError(e)))
    }
}
