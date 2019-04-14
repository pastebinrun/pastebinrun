use crate::models::{Language, Paste};
use crate::schema::{languages, pastes};
use crate::{render, Connection};
use ammonia::Builder;
use askama::Template;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use lazy_static::lazy_static;
use pulldown_cmark::{Options, Parser};
use warp::{Rejection, Reply};

struct TemplatePaste {
    paste: String,
    language_id: i32,
    delete_at: Option<DateTime<Utc>>,
    markdown: String,
}

impl TemplatePaste {
    fn from_paste(paste: Paste) -> Self {
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

#[derive(Template)]
#[template(path = "viewpaste.html")]
struct DisplayPaste {
    languages: Vec<Language>,
    paste: TemplatePaste,
}

pub fn display_paste(
    requested_identifier: String,
    db: Connection,
) -> Result<impl Reply, Rejection> {
    Paste::delete_old(&db);
    let languages = Language::fetch(&db);
    let paste = pastes::table
        .inner_join(languages::table)
        .select((
            pastes::paste,
            pastes::language_id,
            pastes::delete_at,
            languages::is_markdown,
            pastes::no_follow,
        ))
        .filter(pastes::identifier.eq(requested_identifier))
        .get_result(&db)
        .optional()
        .unwrap();
    paste.ok_or_else(warp::reject::not_found).map(|paste| {
        render(DisplayPaste {
            languages,
            paste: TemplatePaste::from_paste(paste),
        })
    })
}
