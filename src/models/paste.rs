use crate::schema::pastes;
use ammonia::Builder;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use lazy_static::lazy_static;
use log::info;
use pulldown_cmark::{Options, Parser};

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
            no_follow,
        } = paste;
        let markdown = if is_markdown {
            render_markdown(&paste, no_follow)
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

fn render_markdown(markdown: &str, no_follow: bool) -> String {
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
    if no_follow {
        FILTER.clean(&output).to_string()
    } else {
        ammonia::clean(&output)
    }
}
