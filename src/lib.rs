use thought_plugin::{
    askama::Template,
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
    author: &'a str,
    asset_prefix: &'a str,
    search_js: &'a str,
    site_title: &'a str,
    footer: &'a str,
}

struct IndexEntry {
    title: String,
    href: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    entries: &'a [IndexEntry],
    search_js: &'a str,
    asset_prefix: &'a str,
    site_title: &'a str,
    footer: &'a str,
    has_entries: bool,
}

impl Theme for Zenflow {
    fn generate_page(article: Article) -> String {
        let html_output = markdown_to_html(article.content());
        let created = format_rfc3339(article.metadata().created());
        let depth = article.preview().category().path().len();
        let asset_prefix = relative_prefix(depth);
        let search_js = search_script_at_depth(depth);

        PageTemplate {
            title: article.title(),
            created: &created,
            author: article.preview().metadata().author(),
            body: html_output.as_str(),
            asset_prefix: &asset_prefix,
            search_js: &search_js,
            site_title: SITE_TITLE,
            footer: FOOTER,
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
        let search_js = search_script_path();
        let asset_prefix = ".";

        IndexTemplate {
            entries: &entries,
            asset_prefix,
            search_js,
            site_title: SITE_TITLE,
            footer: FOOTER,
            has_entries: !entries.is_empty(),
        }
        .render()
        .expect("failed to render index template")
    }
}

export_theme!(Zenflow);

fn relative_prefix(depth: usize) -> String {
    if depth == 0 {
        ".".to_string()
    } else {
        "../".repeat(depth)
    }
}

const SITE_TITLE: &str = "Zenflow";
const FOOTER: &str = "Thought";
const SEARCH_BUNDLE: &str = "assets/thought-search/thought-search.js";

fn search_script_at_depth(depth: usize) -> String {
    let mut prefix = String::new();
    if depth == 0 {
        prefix.push('.');
    } else {
        prefix.push_str(&"../".repeat(depth));
    }
    if !prefix.ends_with('/') {
        prefix.push('/');
    }
    format!("{prefix}{SEARCH_BUNDLE}")
}

fn search_script_path() -> &'static str {
    SEARCH_BUNDLE
}
