use thought_plugin::{
    askama::Template,
    export_theme,
    helpers::{
        article_output_file, format_rfc3339, markdown_to_html, search_script_for_article,
        search_script_path,
    },
    Article, ArticlePreview, Theme,
};

pub struct Zenflow;

#[derive(Template)]
#[template(path = "article.html")]
struct PageTemplate<'a> {
    title: &'a str,
    site_title: &'a str,
    footer: &'a str,
    created: &'a str,
    body: &'a str,
    author: &'a str,
    asset_prefix: &'a str,
    search_js: &'a str,
}

struct IndexEntry {
    title: String,
    href: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    entries: &'a [IndexEntry],
    site_title: &'a str,
    footer: &'a str,
    asset_prefix: &'a str,
    search_js: &'a str,
}

impl Theme for Zenflow {
    fn generate_page(article: Article) -> String {
        let html_output = markdown_to_html(article.content());
        let created = format_rfc3339(article.metadata().created());
        let depth = article.preview().category().path().len();
        let asset_prefix = relative_prefix(depth);
        let search_js = search_script_for_article(article.preview());

        PageTemplate {
            title: article.title(),
            site_title: article.preview().site_title(),
            footer: article.preview().site_footer(),
            created: &created,
            author: article.preview().metadata().author(),
            body: html_output.as_str(),
            asset_prefix: &asset_prefix,
            search_js: &search_js,
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
            site_title: "Zenflow",
            footer: "Thought",
            asset_prefix,
            search_js,
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
