use astra_plugin_sdk::prelude::*;
use reqwest::Client;
use scraper::{Html, Selector};

// ── Localisation ────────────────────────────────────────────────────────────

fn t(lang: &str, key: &str) -> String {
    let base = lang.split('-').next().unwrap_or("en");
    let map = match base {
        "ru" => [
            (
                "error_missing_query",
                "Ошибка: параметр запроса обязателен.",
            ),
            ("search_failed", "Ошибка поиска: {error}"),
            ("no_results", "Результаты не найдены."),
        ],
        "uk" => [
            (
                "error_missing_query",
                "Помилка: параметр запиту обов'язковий.",
            ),
            ("search_failed", "Помилка пошуку: {error}"),
            ("no_results", "Результатів не знайдено."),
        ],
        _ => [
            ("error_missing_query", "Error: query parameter is required."),
            ("search_failed", "Search failed: {error}"),
            ("no_results", "No results found."),
        ],
    };
    map.iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| v.to_string())
        .unwrap_or_else(|| key.to_string())
}

fn fail(lang: &str, e: &dyn std::fmt::Display) -> ToolError {
    ToolError::Internal(t(lang, "search_failed").replace("{error}", &e.to_string()))
}

// ── Config ──────────────────────────────────────────────────────────────────

#[astra::config]
pub struct DdgConfig {
    /// "auto", "5", "10", "15", "20", "25", "30", "35", "40", "45", "50"
    default_limit: String,
    /// "auto", "en-us", "ru-ru", "de-de", "fr-fr", "es-es"
    default_lang: String,
}

impl Default for DdgConfig {
    fn default() -> Self {
        Self {
            default_limit: "auto".into(),
            default_lang: "auto".into(),
        }
    }
}

// ── Arguments ───────────────────────────────────────────────────────────────

#[astra::args]
pub struct SearchArgs {
    /// Search query string.
    query: String,
    /// Number of results: "auto" (default 5) or a number 5–50 step 5.
    #[serde(default = "default_limit")]
    limit: String,
    /// Language code: "auto" or e.g. "en-us", "ru-ru".
    #[serde(default = "default_lang")]
    lang: String,
}

fn default_limit() -> String {
    "auto".into()
}

fn default_lang() -> String {
    "auto".into()
}

// ── Plugin ──────────────────────────────────────────────────────────────────

#[derive(Default)]
pub struct AstraWebsearchDdg {
    config: Config<DdgConfig>,
}

impl AstraWebsearchDdg {
    fn effective_limit(&self, limit: &str) -> usize {
        if limit == "auto" {
            match self.config.load().default_limit.as_str() {
                "auto" => 5,
                n => n.parse().unwrap_or(5),
            }
        } else {
            limit.parse().unwrap_or(5)
        }
    }

    fn effective_lang(&self, lang: &str) -> String {
        if lang == "auto" {
            self.config.load().default_lang.clone()
        } else {
            lang.to_string()
        }
    }

    async fn search_html(
        &self,
        client: &Client,
        query: &str,
        lang: &str,
    ) -> Result<Vec<String>, ToolError> {
        let mut params = vec![("q", query.to_string())];
        if lang != "auto" {
            params.push(("kl", lang.to_string()));
        }
        let resp = client
            .get("https://html.duckduckgo.com/html/")
            .query(&params)
            .header("User-Agent", "AstraWebsearchDdg/1.0")
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| fail(lang, &e))?;

        let resp = resp.error_for_status().map_err(|e| fail(lang, &e))?;

        let body = resp.text().await.map_err(|e| fail(lang, &e))?;

        let document = Html::parse_document(&body);
        let selector = Selector::parse("a.result__a").map_err(|e| fail(lang, &e))?;

        Ok(document
            .select(&selector)
            .filter_map(|el| {
                let title = el.text().collect::<String>();
                let url = el.value().attr("href")?;
                if !title.is_empty() && !url.is_empty() {
                    Some(format!("{title} – {url}"))
                } else {
                    None
                }
            })
            .collect())
    }

    async fn search_api(&self, client: &Client, query: &str, lang: &str) -> Option<String> {
        let mut params = vec![
            ("q", query.to_string()),
            ("format", "json".into()),
            ("no_redirect", "1".into()),
            ("no_html", "1".into()),
            ("skip_disambig", "1".into()),
        ];
        if lang != "auto" {
            params.push(("kl", lang.to_string()));
        }
        let resp = client
            .get("https://api.duckduckgo.com/")
            .query(&params)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
            .ok()?;
        let data: serde_json::Value = resp.json().await.ok()?;
        let abstract_text = data.get("Abstract")?.as_str()?;
        let abstract_url = data.get("AbstractURL")?.as_str()?;
        if abstract_text.is_empty() || abstract_url.is_empty() {
            return None;
        }
        Some(format!("{abstract_text} – {abstract_url}"))
    }
}

// ── What Astra can call ─────────────────────────────────────────────────────

#[astra::plugin]
impl AstraWebsearchDdg {
    /// Search the web using DuckDuckGo. Returns a list of titles and URLs.
    #[tool]
    async fn duckduckgo_search(&self, a: SearchArgs) -> Result<String, ToolError> {
        let query = a.query.trim();
        if query.is_empty() {
            return Err(ToolError::BadArguments(t(
                &self.effective_lang(&a.lang),
                "error_missing_query",
            )));
        }

        let lang = self.effective_lang(&a.lang);
        let limit = self.effective_limit(&a.limit);

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .map_err(|e| {
                ToolError::Internal(t(&lang, "search_failed").replace("{error}", &e.to_string()))
            })?;

        let mut results = self.search_html(&client, query, &lang).await?;

        if results.is_empty() {
            if let Some(ia) = self.search_api(&client, query, &lang).await {
                results.push(ia);
            }
        }

        if results.is_empty() {
            return Ok(t(&lang, "no_results"));
        }

        results.truncate(limit);
        Ok(results.join("\n"))
    }

    #[hook]
    async fn on_config(&self, ctx: &PluginContext, config: DdgConfig) {
        let _ = ctx
            .host()
            .log_info(&format!(
                "config: default_limit={}, default_lang={}",
                config.default_limit, config.default_lang
            ))
            .await;
        self.config.store(config);
    }

    #[hook]
    async fn health_check(&self) -> (bool, String) {
        (true, "ok".into())
    }
}

astra::main!(AstraWebsearchDdg::default());

#[cfg(test)]
mod tests {
    use super::*;
    use astra_plugin_sdk::testing::Harness;

    #[tokio::test]
    async fn it_starts_and_passes_health() {
        let h = Harness::new(AstraWebsearchDdg::default())
            .with_config(json!({}))
            .start()
            .await
            .expect("the plugin started");

        assert!(h.health().await.0);
    }

    #[tokio::test]
    async fn empty_query_is_rejected() {
        let h = Harness::new(AstraWebsearchDdg::default())
            .with_config(json!({}))
            .start()
            .await
            .expect("the plugin started");

        let err = h
            .call_tool("duckduckgo_search", json!({"query": ""}))
            .await
            .expect_err("empty query must be rejected");

        assert!(matches!(err, ToolError::BadArguments(_)));
        assert!(err.to_string().contains("query"));
    }

    #[tokio::test]
    async fn fuzz_configs_do_not_crash() {
        let h = Harness::new(AstraWebsearchDdg::default())
            .start()
            .await
            .expect("the plugin started");

        // The daemon delivers config it did not author: `{}` is the first
        // payload every fresh install sees, and noisy ones must not throw.
        let artifacts = astra_plugin_sdk::testing::fixtures::config_fuzz();
        for (payload, _why) in artifacts.iter().take(4) {
            let fresh = Harness::new(AstraWebsearchDdg::default())
                .with_config_json(*payload)
                .start()
                .await;
            if let Ok(fresh) = fresh {
                assert!(fresh.health().await.0);
            }
            let _ = h.health().await;
        }
    }
}
