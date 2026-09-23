use askama::Template;
use thought_plugin::{
    export_theme,
    helpers::{index_assets_prefix, index_search_script_path},
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
    translations: &'a [LangOption],
    has_translations: bool,
}

struct IndexEntry {
    title: String,
    href: String,
    locale: String,
    default_locale: String,
}

struct LangOption {
    locale: String,
    href: String,
    selected: bool,
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
    locales: Vec<String>,
    locales_json: String,
    has_locale_switch: bool,
}

impl Theme for Zenflow {
    fn generate_page(article: Article) -> String {
        let html_output = article.content_html();
        let created = article.metadata().created_display_for(article.locale());
        let asset_prefix = article.assets_prefix();
        let search_js = article.search_script_path();
        let translations = article
            .translation_links()
            .into_iter()
            .map(|link| LangOption {
                selected: link.locale == article.locale(),
                locale: link.locale,
                href: link.href,
            })
            .collect::<Vec<_>>();

        PageTemplate {
            title: article.title(),
            created: &created,
            author: article.preview().metadata().author(),
            body: html_output.as_str(),
            asset_prefix: asset_prefix.as_str(),
            search_js: search_js.as_str(),
            site_title: SITE_TITLE,
            footer: FOOTER,
            translations: translations.as_slice(),
            has_translations: translations.len() > 1,
        }
        .render()
        .expect("failed to render page template")
    }

    fn generate_index(articles: Vec<ArticlePreview>) -> String {
        let mut entries = Vec::new();
        let mut locales = std::collections::BTreeSet::new();
        for article in articles {
            locales.insert(article.default_locale().to_owned());
            locales.insert(article.locale().to_owned());
            for translation in article.translations() {
                locales.insert(translation.locale.to_owned());
            }
            entries.push(IndexEntry {
                title: article.title().to_owned(),
                href: article.output_file(),
                locale: article.locale().to_owned(),
                default_locale: article.default_locale().to_owned(),
            });
        }
        let locales: Vec<String> = locales.into_iter().collect();
        let locales_json = serde_json::to_string(&locales).unwrap_or_else(|_| "[]".to_string());
        let has_locale_switch = locales.len() > 1;
        let search_js = index_search_script_path();
        let asset_prefix = index_assets_prefix();

        IndexTemplate {
            entries: &entries,
            asset_prefix,
            search_js,
            site_title: SITE_TITLE,
            footer: FOOTER,
            has_entries: !entries.is_empty(),
            locales,
            locales_json,
            has_locale_switch,
        }
        .render()
        .expect("failed to render index template")
    }
}

export_theme!(Zenflow);

const SITE_TITLE: &str = "Zenflow";
const FOOTER: &str = "Thought";
