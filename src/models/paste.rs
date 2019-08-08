use crate::schema::pastes;
use crate::PgPool;
use ammonia::Builder;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use futures::Future;
use futures03::TryFutureExt;
use lazy_static::lazy_static;
use log::info;
use pulldown_cmark::{Options, Parser};
use tokio_diesel::{AsyncError, AsyncRunQueryDsl};

#[derive(Queryable)]
pub struct Paste {
    pub paste: String,
    pub language_id: i32,
    pub delete_at: Option<DateTime<Utc>>,
    pub is_markdown: bool,
}

impl Paste {
    pub fn delete_old(pool: &'static PgPool) -> impl Future<Item = (), Error = AsyncError> {
        diesel::delete(pastes::table)
            .filter(pastes::delete_at.lt(Utc::now()))
            .execute_async(pool)
            .compat()
            .map(|pastes| {
                if pastes > 0 {
                    info!("Deleted {} paste(s)", pastes);
                }
            })
    }
}

pub struct ExternPaste {
    pub paste: String,
    pub language_id: i32,
    pub delete_at: Option<DateTime<Utc>>,
    pub markdown: String,
}

impl ExternPaste {
    pub fn from_paste(paste: Paste) -> Self {
        let Paste {
            paste,
            language_id,
            delete_at,
            is_markdown,
        } = paste;
        let markdown = if is_markdown {
            render_markdown(&paste)
        } else {
            String::new()
        };
        Self {
            paste,
            language_id,
            delete_at,
            markdown,
        }
    }
}

fn render_markdown(markdown: &str) -> String {
    lazy_static! {
        static ref FILTER: Builder<'static> = {
            let mut builder = Builder::new();
            builder.link_rel(Some("noopener noreferrer nofollow"));
            builder
        };
    }
    let mut output = String::new();
    pulldown_cmark::html::push_html(
        &mut output,
        Parser::new_ext(markdown, Options::ENABLE_TABLES),
    );
    FILTER.clean(&output).to_string()
}
