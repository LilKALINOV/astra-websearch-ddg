"""AstraWebsearchDdg — Astra plugin."""

from astra_plugin_sdk import Plugin, tool
from typing import Literal

# Simple in‑code localisation mapping (fallback to English)
_LOCALIZED = {
    "en": {
        "error_missing_query": "Error: query parameter is required.",
        "search_failed": "Search failed: {error}",
        "no_results": "No results found.",
    },
    "ru": {
        "error_missing_query": "Ошибка: параметр запроса обязателен.",
        "search_failed": "Ошибка поиска: {error}",
        "no_results": "Результаты не найдены.",
    },
    "uk": {
        "error_missing_query": "Помилка: параметр запиту обов'язковий.",
        "search_failed": "Помилка пошуку: {error}",
        "no_results": "Результатів не знайдено.",
    },
}

def _t(lang_code: str, key: str, **kwargs) -> str:
    # Use the user‑preferred language if available, otherwise fall back to English.
    msgs = _LOCALIZED.get(lang_code, _LOCALIZED["en"])
    template = msgs.get(key, _LOCALIZED["en"].get(key, key))
    return template.format(**kwargs)



class AstraWebsearchDdg(Plugin):
    """Astra plugin: astra-websearch-ddg."""

    @tool("Search the web using DuckDuckGo Instant Answer API. Returns a short list of titles and URLs.")
    async def duckduckgo_search(self, query: str = "", limit: Literal["auto", 5, 10, 15, 20, 25, 30, 35, 40, 45, 50] = "auto", lang: Literal["auto", "en-us", "ru-ru", "de-de", "fr-fr", "es-es"] = "auto"):
        """Perform a web search.
        
        Parameters
        ----------
        query: str
            Search query string.
        limit: "auto" or int (5‑50 step 5)
            Number of results to return. "auto" uses the plugin default (5).
        lang: str
            Language code for DuckDuckGo (default "auto").
        """
        import requests
        if not query:
            # Use the user‑preferred language for the error message (fallback to English)
            return _t(lang.split('-')[0] if lang != "auto" else "en", "error_missing_query")

        results = []  # accumulator for output lines
        url = "https://html.duckduckgo.com/html/"
        params = {
            "q": query,
        }
        # language selection – DuckDuckGo uses the "kl" parameter for HTML endpoint as well.
        if lang != "auto":
            params["kl"] = lang
        try:
            # The HTML endpoint returns UTF‑8 text; we request the page and parse it with the stdlib parser.
            resp = requests.get(url, params=params, timeout=10, headers={"User-Agent": "AstraWebsearchDdg/1.0"})
            resp.raise_for_status()
        except Exception as e:
            return _t(lang.split('-')[0] if lang != "auto" else "en", "search_failed", error=e)
        # Parse the HTML to extract titles and URLs. Each result is an <a class="result__a" href="…">Title</a>.
        from html.parser import HTMLParser
        class _ResultParser(HTMLParser):
            def __init__(self):
                super().__init__()
                self.results = []
                self._capture = False
                self._url = None
                self._text_parts = []

            def handle_starttag(self, tag, attrs):
                if tag == "a":
                    attrs_dict = dict(attrs)
                    if attrs_dict.get("class", "").find("result__a") != -1:
                        self._capture = True
                        self._url = attrs_dict.get("href")
                        self._text_parts = []

            def handle_endtag(self, tag):
                if tag == "a" and self._capture:
                    title = "".join(self._text_parts).strip()
                    if title and self._url:
                        self.results.append(f"{title} – {self._url}")
                    self._capture = False
                    self._url = None
                    self._text_parts = []

            def handle_data(self, data):
                if self._capture:
                    self._text_parts.append(data)
        parser = _ResultParser()
        parser.feed(resp.text)
        results.extend(parser.results)
        # If the HTML parser found nothing, fall back to the Instant Answer API (good for definitions)
        if not results:
            # Instant Answer fallback – may provide a concise abstract.
            api_url = "https://api.duckduckgo.com/"
            api_params = {
                "q": query,
                "format": "json",
                "no_redirect": 1,
                "no_html": 1,
                "skip_disambig": 1,
            }
            if lang != "auto":
                api_params["kl"] = lang
            try:
                api_resp = requests.get(api_url, params=api_params, timeout=10)
                api_resp.raise_for_status()
                api_data = api_resp.json()
                abstract = api_data.get("Abstract")
                abstract_url = api_data.get("AbstractURL")
                if abstract and abstract_url:
                    results.append(f"{abstract} – {abstract_url}")
            except Exception:
                pass



if __name__ == "__main__":
    AstraWebsearchDdg().run()
