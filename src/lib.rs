use askama::Template;
use thought_plugin::{
    export_theme,
    helpers::{article_output_file, format_rfc3339, markdown_to_html},
    Article, ArticlePreview, Theme,
};

pub struct Zenflow;

#[derive(Template)]
#[template(path = "article.html")]
struct PageTemplate<'a> {
    title: &'a str,
    created: &'a str,
    body: &'a str,
}

struct IndexEntry {
    title: String,
    href: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    entries: &'a [IndexEntry],
}

impl Theme for Zenflow {
    fn generate_page(article: Article) -> String {
        let html_output = markdown_to_html(article.content());
        let created = format_rfc3339(article.metadata().created());

        PageTemplate {
            title: article.title(),
            created: &created,
            body: html_output.as_str(),
        }
        .render()
        .expect("failed to render page template")
    }

    fn generate_index(articles: Vec<ArticlePreview>) -> String {
        let mut entries = Vec::new();
        for article in articles {
            entries.push(IndexEntry {
                title: article.title().to_owned(),
                href: article_output_file(&article),
            });
        }

        IndexTemplate { entries: &entries }
            .render()
            .expect("failed to render index template")
    }
}

export_theme!(Zenflow);
