use crate::schema::pastes;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use log::info;

#[derive(Queryable)]
pub struct Paste {
    pub paste: String,
    pub language_id: i32,
    pub delete_at: Option<DateTime<Utc>>,
    pub is_markdown: bool,
    pub no_follow: bool,
}

impl Paste {
    pub fn delete_old(db: &PgConnection) {
        let pastes = diesel::delete(pastes::table)
            .filter(pastes::delete_at.lt(Utc::now()))
            .execute(db)
            .unwrap();
        if pastes > 0 {
            info!("Deleted {} paste(s)", pastes);
        }
    }
}
