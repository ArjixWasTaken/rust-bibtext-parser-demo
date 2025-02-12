use std::collections::VecDeque;

use crate::types::{Article, Block, Book, InProceedings};

pub struct Parser {
    lines: VecDeque<String>,
}

impl Parser {
    pub fn new<S: AsRef<str>>(text: S) -> Self {
        Self {
            lines: text
                .as_ref()
                .lines()
                .filter_map(|x| {
                    let x = x.trim();
                    if x.is_empty() {
                        None
                    } else {
                        Some(x.to_string())
                    }
                })
                .collect(),
        }
    }

    pub fn parse(&mut self) -> Vec<Block> {
        let mut blocks = vec![];

        while !self.lines.is_empty() {
            self.burn_comments();

            let Some(block) = self.parse_block() else {
                break;
            };

            blocks.push(block);
        }

        blocks
    }

    pub fn parse_block(&mut self) -> Option<Block> {
        let mut block = String::new();

        loop {
            let line = self.lines.pop_front().unwrap();
            block.push_str(&line);

            if line.contains("}") {
                break;
            }
        }

        let block = block.trim();
        let (block_type, rest) = block.split_at(block.find("{").unwrap());

        match block_type {
            "@article" => Some(self.parse_article(rest)),
            "@inproceedings" => Some(self.parse_inproceedings(rest)),
            "@Book" => Some(self.parse_book(rest)),
            _ => unimplemented!("Unknown block type: {}", block_type),
        }
    }

    pub fn parse_article(&mut self, rest: &str) -> Block {
        // NOTE: Assuming that `rest` starts with "{" and ends with "}"

        let rest = &rest[1..rest.len() - 1];
        let (id, rest) = rest.split_at(rest.find(",").unwrap());

        let mut rest = rest[1..].to_string();

        let mut article = Article {
            id: id.to_string(),
            ..Default::default()
        };

        loop {
            let (pair, rest_) = self.parse_key_value(rest);
            rest = rest_;

            let Some((key, value)) = pair else {
                break;
            };

            match key.as_str() {
                "author" => article.author = Some(value),
                "title" => article.title = Some(value),
                "journal" => article.journal = Some(value),
                "year" => article.year = Some(value.parse().unwrap()),
                "volume" => article.volume = Some(value),
                "number" => article.number = Some(value),
                "pages" => article.pages = Some(value),
                "month" => article.month = Some(value),
                "doi" => article.doi = Some(value),
                "note" => article.note = Some(value),
                _ => unimplemented!("Unknown key: {}", key),
            }
        }

        return Block::Article(article);
    }

    pub fn parse_inproceedings(&mut self, rest: &str) -> Block {
        // NOTE: Assuming that `rest` starts with "{" and ends with "}"

        let rest = &rest[1..rest.len() - 1];
        let (id, rest) = rest.split_at(rest.find(",").unwrap());

        let mut rest = rest[1..].to_string();

        let mut in_proceedings = InProceedings {
            id: id.to_string(),
            ..Default::default()
        };

        loop {
            let (pair, rest_) = self.parse_key_value(rest);
            rest = rest_;

            let Some((key, value)) = pair else {
                break;
            };

            match key.as_str() {
                "author" => in_proceedings.author = Some(value),
                "title" => in_proceedings.title = Some(value),
                "booktitle" => in_proceedings.booktitle = Some(value),
                "year" => in_proceedings.year = Some(value.parse().unwrap()),
                "editor" => in_proceedings.editor = Some(value),
                "volume" => in_proceedings.volume = Some(value),
                "number" => in_proceedings.number = Some(value),
                "series" => in_proceedings.series = Some(value),
                "pages" => in_proceedings.pages = Some(value),
                "address" => in_proceedings.address = Some(value),
                "month" => in_proceedings.month = Some(value),
                "organization" => in_proceedings.organization = Some(value),
                "publisher" => in_proceedings.publisher = Some(value),
                _ => unimplemented!("Unknown key: {}", key),
            }
        }

        return Block::InProceedings(in_proceedings);
    }

    pub fn parse_book(&mut self, rest: &str) -> Block {
        // NOTE: Assuming that `rest` starts with "{" and ends with "}"

        let rest = &rest[1..rest.len() - 1];
        let (id, rest) = rest.split_at(rest.find(",").unwrap());

        let mut rest = rest[1..].to_string();

        let mut book = Book {
            id: id.to_string(),
            ..Default::default()
        };

        loop {
            let (pair, rest_) = self.parse_key_value(rest);
            rest = rest_;

            let Some((key, value)) = pair else {
                break;
            };

            match key.as_str() {
                "author" => book.author = Some(value),
                "title" => book.title = Some(value),
                "publisher" => book.publisher = Some(value),
                "year" => book.year = Some(value.parse().unwrap()),
                "address" => book.address = Some(value),
                "pages" => book.pages = Some(value),
                "edition" => book.edition = Some(value),
                "isbn" => book.isbn = Some(value),
                "note" => book.note = Some(value),
                _ => unimplemented!("Unknown key: {}", key),
            }
        }

        return Block::Book(book);
    }

    pub fn parse_key_value(&mut self, rest: String) -> (Option<(String, String)>, String) {
        if rest.trim().trim_matches(',').is_empty() {
            return (None, rest);
        }

        let equals_idx = rest.find("=");
        if equals_idx.is_none() {
            return (None, rest);
        }

        let (key, rest) = rest.split_at(equals_idx.unwrap());

        let key = key.trim().to_string();
        let mut rest = rest[1..].trim();

        let mut value = String::new();

        if rest.starts_with("\"") {
            let mut string = String::new();
            let chars = rest.chars().collect::<Vec<char>>();
            let mut idx = 1;

            let mut prev = chars.get(idx).unwrap_or(&'.');

            while idx < chars.len() {
                if chars[idx] == '"' {
                    if prev != &'\\' {
                        idx += 1;
                        break;
                    }
                }

                if prev == &'\\' {
                    match chars[idx] {
                        // Replaces some escaped characters with their actual value.
                        'n' => {
                            string.pop();
                            string.push('\n');
                        }
                        't' => {
                            string.pop();
                            string.push('\t');
                        }
                        'r' => {
                            string.pop();
                            string.push('\r');
                        }
                        '"' => {
                            string.pop();
                            string.push('"');
                        }
                        _ => string.push(chars[idx]),
                    }
                } else {
                    string.push(chars[idx])
                }

                prev = &chars[idx];
                idx += 1;
            }

            return (
                Some((key, string.clone())),
                chars[idx..]
                    .into_iter()
                    .map(|x| x.clone())
                    .collect::<String>()
                    .trim_matches(',')
                    .to_string(),
            );
        }

        let split = rest.split_at(rest.find(",").unwrap());
        value = split.0.trim().to_string();
        rest = &split.1[1..].trim();

        (Some((key, value)), rest.to_string())
    }

    pub fn burn_comments(&mut self) {
        loop {
            let Some(line) = self.lines.front() else {
                break;
            };

            if line.starts_with("@") || line.starts_with("%") {
                break;
            }

            self.lines.pop_front();
        }
    }
}
