#[derive(Debug, Default)]
pub struct Article {
    pub id: String,
    pub author: Option<String>,
    pub title: Option<String>,
    pub journal: Option<String>,
    pub year: Option<u32>,
    pub volume: Option<String>,
    pub number: Option<String>,
    pub pages: Option<String>,
    pub month: Option<String>,
    pub doi: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Default)]
pub struct InProceedings {
    pub id: String,
    pub author: Option<String>,
    pub title: Option<String>,
    pub booktitle: Option<String>,
    pub year: Option<u32>,
    pub editor: Option<String>,
    pub volume: Option<String>,
    pub number: Option<String>,
    pub series: Option<String>,
    pub pages: Option<String>,
    pub address: Option<String>,
    pub month: Option<String>,
    pub organization: Option<String>,
    pub publisher: Option<String>,
}

#[derive(Debug, Default)]
pub struct Book {
    pub id: String,
    pub author: Option<String>,
    pub title: Option<String>,
    pub publisher: Option<String>,
    pub year: Option<u32>,
    pub address: Option<String>,
    pub edition: Option<String>,
    pub pages: Option<String>,
    pub isbn: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug)]
pub enum Block {
    // @article
    Article(Article),

    // @inproceedings
    InProceedings(InProceedings),

    // @Book
    Book(Book),
}

impl Block {
    pub fn encode(&self) -> String {
        match self {
            Block::Article(article) => article.encode(),
            Block::InProceedings(inproceedings) => inproceedings.encode(),
            Block::Book(book) => book.encode(),
        }
    }
}

impl Article {
    pub fn encode(&self) -> String {
        let mut s = format!("@article{{{id},\n", id = self.id);
        if let Some(author) = &self.author {
            s.push_str(&format!("    author = \"{author}\",\n", author = author));
        }
        if let Some(title) = &self.title {
            s.push_str(&format!("    title = \"{title}\",\n", title = title));
        }
        if let Some(journal) = &self.journal {
            s.push_str(&format!(
                "    journal = \"{journal}\",\n",
                journal = journal
            ));
        }
        if let Some(year) = &self.year {
            s.push_str(&format!("    year = \"{year}\",\n", year = year));
        }
        if let Some(volume) = &self.volume {
            s.push_str(&format!("    volume = \"{volume}\",\n", volume = volume));
        }
        if let Some(number) = &self.number {
            s.push_str(&format!("    number = \"{number}\",\n", number = number));
        }
        if let Some(pages) = &self.pages {
            s.push_str(&format!("    pages = \"{pages}\",\n", pages = pages));
        }
        if let Some(month) = &self.month {
            s.push_str(&format!("    month = \"{month}\",\n", month = month));
        }
        if let Some(doi) = &self.doi {
            s.push_str(&format!("    doi = \"{doi}\",\n", doi = doi));
        }
        if let Some(note) = &self.note {
            s.push_str(&format!("    note = \"{note}\",\n", note = note));
        }
        s.push_str("}\n");
        s
    }
}

impl InProceedings {
    pub fn encode(&self) -> String {
        let mut s = format!("@inproceedings{{{id},\n", id = self.id);
        if let Some(author) = &self.author {
            s.push_str(&format!("    author = \"{author}\",\n", author = author));
        }
        if let Some(title) = &self.title {
            s.push_str(&format!("    title = \"{title}\",\n", title = title));
        }
        if let Some(booktitle) = &self.booktitle {
            s.push_str(&format!(
                "    booktitle = \"{booktitle}\",\n",
                booktitle = booktitle
            ));
        }
        if let Some(year) = &self.year {
            s.push_str(&format!("    year = \"{year}\",\n", year = year));
        }
        if let Some(editor) = &self.editor {
            s.push_str(&format!("    editor = \"{editor}\",\n", editor = editor));
        }
        if let Some(volume) = &self.volume {
            s.push_str(&format!("    volume = \"{volume}\",\n", volume = volume));
        }
        if let Some(number) = &self.number {
            s.push_str(&format!("    number = \"{number}\",\n", number = number));
        }
        if let Some(series) = &self.series {
            s.push_str(&format!("    series = \"{series}\",\n", series = series));
        }
        if let Some(pages) = &self.pages {
            s.push_str(&format!("    pages = \"{pages}\",\n", pages = pages));
        }
        if let Some(address) = &self.address {
            s.push_str(&format!(
                "    address = \"{address}\",\n",
                address = address
            ));
        }
        if let Some(month) = &self.month {
            s.push_str(&format!("    month = \"{month}\",\n", month = month));
        }
        if let Some(organization) = &self.organization {
            s.push_str(&format!(
                "    organization = \"{organization}\",\n",
                organization = organization
            ));
        }
        if let Some(publisher) = &self.publisher {
            s.push_str(&format!(
                "    publisher = \"{publisher}\",\n",
                publisher = publisher
            ));
        }

        s
    }
}

impl Book {
    pub fn encode(&self) -> String {
        let mut s = format!("@Book{{{id},\n", id = self.id);
        if let Some(author) = &self.author {
            s.push_str(&format!("    author = \"{author}\",\n", author = author));
        }
        if let Some(title) = &self.title {
            s.push_str(&format!("    title = \"{title}\",\n", title = title));
        }
        if let Some(publisher) = &self.publisher {
            s.push_str(&format!(
                "    publisher = \"{publisher}\",\n",
                publisher = publisher
            ));
        }
        if let Some(year) = &self.year {
            s.push_str(&format!("    year = \"{year}\",\n", year = year));
        }
        if let Some(address) = &self.address {
            s.push_str(&format!(
                "    address = \"{address}\",\n",
                address = address
            ));
        }
        if let Some(edition) = &self.edition {
            s.push_str(&format!(
                "    edition = \"{edition}\",\n",
                edition = edition
            ));
        }
        if let Some(pages) = &self.pages {
            s.push_str(&format!("    pages = \"{pages}\",\n", pages = pages));
        }
        if let Some(isbn) = &self.isbn {
            s.push_str(&format!("    isbn = \"{isbn}\",\n", isbn = isbn));
        }
        if let Some(note) = &self.note {
            s.push_str(&format!("    note = \"{note}\",\n", note = note));
        }
        s.push_str("}\n");
        s
    }
}
