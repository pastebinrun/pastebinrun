// pastebin.run
// Copyright (C) 2020 Konrad Borowski
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::schema::{languages, pastes};
use ammonia::Builder;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use itertools::Itertools;
use log::info;
use once_cell::sync::Lazy;
use pulldown_cmark::{Options, Parser};
use rand::seq::SliceRandom;
use rocket::http::Status;
use rocket::response::{self, Debug, Responder};
use rocket::Request;
use std::iter;

#[derive(Queryable)]
pub struct Paste {
    pub identifier: String,
    pub paste: String,
    pub language_id: i32,
    pub delete_at: Option<DateTime<Utc>>,
    pub language_identifier: String,
    pub stdin: String,
    pub exit_code: Option<i32>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

impl Paste {
    pub fn delete_old(connection: &PgConnection) -> Result<(), diesel::result::Error> {
        let pastes = diesel::delete(pastes::table)
            .filter(pastes::delete_at.lt(Utc::now()))
            .execute(connection)?;
        if pastes > 0 {
            info!("Deleted {} paste(s)", pastes);
        }
        Ok(())
    }
}

const CHARACTERS: &[u8] = b"23456789bcdfghjkmnpqrstvwxz-";

#[derive(Insertable)]
#[table_name = "pastes"]
struct InsertPaste<'a> {
    identifier: &'a str,
    delete_at: Option<DateTime<Utc>>,
    language_id: i32,
    paste: &'a str,
    stdin: &'a str,
    stdout: Option<&'a str>,
    stderr: Option<&'a str>,
    exit_code: Option<i32>,
}

#[derive(Default)]
pub struct ExtraPasteParameters<'a> {
    pub stdin: &'a str,
    pub stdout: Option<&'a str>,
    pub stderr: Option<&'a str>,
    pub exit_code: Option<i32>,
}

pub enum InsertionError {
    Diesel(diesel::result::Error),
    UnrecognizedLanguageIdentifier,
}

impl From<diesel::result::Error> for InsertionError {
    fn from(e: diesel::result::Error) -> Self {
        Self::Diesel(e)
    }
}

impl<'r> Responder<'r, 'static> for InsertionError {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        match self {
            Self::Diesel(e) => Debug(e).respond_to(request),
            Self::UnrecognizedLanguageIdentifier => Err(Status::BadRequest),
        }
    }
}

pub fn insert(
    connection: &PgConnection,
    delete_at: Option<DateTime<Utc>>,
    language: &str,
    paste: &str,
    ExtraPasteParameters {
        stdin,
        stdout,
        stderr,
        exit_code,
    }: ExtraPasteParameters,
) -> Result<String, InsertionError> {
    let mut rng = rand::thread_rng();
    let identifier: String = (0..12)
        .map(|_| char::from(*CHARACTERS.choose(&mut rng).expect("a random character")))
        .collect();
    let language_id = languages::table
        .select(languages::language_id)
        .filter(languages::identifier.eq(language))
        .get_result(connection)
        .optional()?
        .ok_or(InsertionError::UnrecognizedLanguageIdentifier)?;
    let insert_paste = InsertPaste {
        identifier: &identifier,
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
        .execute(connection)?;
    Ok(identifier)
}

#[derive(Default)]
pub struct ExternPaste {
    pub identifier: Option<String>,
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
            identifier,
            paste,
            language_id,
            language_identifier,
            delete_at,
            stdin,
            exit_code,
            stdout,
            stderr,
        } = paste;
        let markdown = if language_identifier == "markdown" {
            render_markdown(&paste)
        } else {
            String::new()
        };
        Self {
            identifier: Some(identifier),
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
    static FILTER: Lazy<Builder<'static>> = Lazy::new(|| {
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
    });
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
