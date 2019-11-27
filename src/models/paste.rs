use crate::models::rejection::CustomRejection;
use crate::schema::{languages, pastes};
use crate::Connection;
use ammonia::Builder;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use itertools::Itertools;
use lazy_static::lazy_static;
use log::info;
use pulldown_cmark::{Options, Parser};
use rand::seq::SliceRandom;
use std::iter;
use warp::Rejection;

#[derive(Queryable)]
pub struct Paste {
    pub paste: String,
    pub language_id: i32,
    pub delete_at: Option<DateTime<Utc>>,
    pub identifier: String,
    pub stdin: String,
    pub exit_code: Option<i32>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
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

const CHARACTERS: &[u8] = b"23456789bcdfghjkmnpqrstvwxzBCDFGHJKLMNPQRSTVWX-";

#[derive(Insertable)]
#[table_name = "pastes"]
struct InsertPaste {
    identifier: String,
    delete_at: Option<DateTime<Utc>>,
    language_id: i32,
    paste: String,
    stdin: String,
    stdout: Option<String>,
    stderr: Option<String>,
    exit_code: Option<i32>,
}

pub fn insert(
    connection: &Connection,
    delete_at: Option<DateTime<Utc>>,
    language: &str,
    paste: String,
    stdin: String,
    stdout: Option<String>,
    stderr: Option<String>,
    exit_code: Option<i32>,
) -> Result<String, Rejection> {
    let mut rng = rand::thread_rng();
    let identifier: String = (0..10)
        .map(|_| char::from(*CHARACTERS.choose(&mut rng).expect("a random character")))
        .collect();
    let language_id = languages::table
        .select(languages::language_id)
        .filter(languages::identifier.eq(language))
        .get_result(connection)
        .optional()
        .map_err(warp::reject::custom)?
        .ok_or_else(|| warp::reject::custom(CustomRejection::UnrecognizedLanguageIdentifier))?;
    for (field, name) in &[(&paste, "paste"), (&stdin, "stdin")] {
        if field.len() > 1_000_000 {
            return Err(warp::reject::custom(CustomRejection::FieldTooLarge(name)));
        }
    }
    for (field, name) in &[(&stdout, "stdout"), (&stderr, "stderr")] {
        if let Some(field) = field {
            if field.len() > 1_000_000 {
                return Err(warp::reject::custom(CustomRejection::FieldTooLarge(name)));
            }
        }
    }
    let insert_paste = InsertPaste {
        identifier,
        delete_at,
        language_id,
        paste,
        stdin,
        stdout,
        stderr,
        exit_code,
    };
    diesel::insert_into(pastes::table)
        .values(&insert_paste)
        .execute(connection)
        .map_err(warp::reject::custom)?;
    Ok(insert_paste.identifier)
}

pub struct ExternPaste {
    pub paste: String,
    pub language_id: i32,
    pub delete_at: Option<DateTime<Utc>>,
    pub markdown: String,
    pub stdin: String,
    pub exit_code: Option<i32>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

impl ExternPaste {
    pub fn from_paste(paste: Paste) -> Self {
        let Paste {
            paste,
            language_id,
            identifier,
            delete_at,
            stdin,
            exit_code,
            stdout,
            stderr,
        } = paste;
        let markdown = if identifier == "markdown" {
            render_markdown(&paste)
        } else {
            String::new()
        };
        Self {
            paste,
            language_id,
            delete_at,
            markdown,
            stdin,
            exit_code,
            stdout,
            stderr,
        }
    }
}

fn render_markdown(markdown: &str) -> String {
    lazy_static! {
        static ref FILTER: Builder<'static> = {
            let mut builder = Builder::new();
            builder.link_rel(Some("noopener noreferrer nofollow"));
            builder.add_generic_attributes(iter::once("class"));
            builder.attribute_filter(|_, attribute, value| {
                if attribute == "class" {
                    Some(
                        value
                            .split_ascii_whitespace()
                            .filter(|value| value.starts_with("language-"))
                            .join(" ")
                            .into(),
                    )
                } else {
                    Some(value.into())
                }
            });
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

    #[test]
    fn code_blocks_work() {
        assert_eq!(
            render_markdown("```rust\nfn main() {}\n```"),
            "<pre><code class=\"language-rust\">fn main() {}\n</code></pre>\n",
        );
    }

    #[test]
    fn only_language_classes_are_allowed() {
        assert_eq!(
            render_markdown(r#"<br class="language-a madoka language-b homura">"#),
            "<br class=\"language-a language-b\">",
        );
    }
}
