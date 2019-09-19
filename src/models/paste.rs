use crate::schema::pastes;
use crate::Connection;
use ammonia::Builder;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use lazy_static::lazy_static;
use log::info;
use pulldown_cmark::{Options, Parser};
use warp::Rejection;

#[derive(Queryable)]
pub struct Paste {
    pub paste: String,
    pub language_id: i32,
    pub delete_at: Option<DateTime<Utc>>,
    pub is_markdown: bool,
}

impl Paste {
    pub fn delete_old(connection: &Connection) -> Result<(), Rejection> {
        let pastes = diesel::delete(pastes::table)
            .filter(pastes::delete_at.lt(Utc::now()))
            .execute(connection)
            .map_err(warp::reject::custom)?;
        if pastes > 0 {
            info!("Deleted {} paste(s)", pastes);
        }
        Ok(())
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
    let options = Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH;
    pulldown_cmark::html::push_html(&mut output, Parser::new_ext(markdown, options));
    FILTER.clean(&output).to_string()
}

#[cfg(test)]
mod test {
    use super::render_markdown;

    #[test]
    fn markdown_works() {
        assert_eq!(
            render_markdown("**bold**"),
            "<p><strong>bold</strong></p>\n"
        );
    }

    #[test]
    fn strikethrough_works() {
        assert_eq!(render_markdown("~~strike~~"), "<p><del>strike</del></p>\n");
    }
}
