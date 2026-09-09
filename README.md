# DuckDuckGo Web Search plugin for Astra

## Overview

**Astra‑Websearch‑Ddg** is a lightweight Astra tool plugin that enables the assistant to perform real‑time web searches via DuckDuckGo.  It returns a concise list of result titles with their URLs, making it easy for the assistant to surface up‑to‑date information to the user.

## Features

- **Full‑text web search** using DuckDuckGo’s HTML endpoint – works for any query, not only instant‑answer topics.
- **Configurable result limit** – the UI offers a dropdown with `auto` (default 5) and the values `5, 10, 15, 20, 25, 30, 35, 40, 45, 50`.
- **Language selection** – a dropdown contains `auto` (let DuckDuckGo decide) and the language codes `en‑us, ru‑ru, de‑de, fr‑fr, es‑es`.
- **Fallback to Instant Answer API** – when the HTML search returns no results (e.g., definitions), the plugin falls back to DuckDuckGo’s JSON API to provide a concise abstract.
- **No UI elements** – the plugin works purely as a **tool**, so it does not add tabs or panels to the Astra window.

## Installation (Development mode)

1. **Enable unsigned plugins** in Astra: `Settings → Privacy → “Allow unsigned plugins”`.
2. **Load the plugin folder** (absolute path) via `Plugins → Dev → Load a plugin`:
   ```
   D:\Plagin\astra\Search\astra-websearch-ddg
   ```
3. After loading, the plugin appears on the **Installed** tab – enable it.

## Usage in chat

The tool is called `duckduckgo_search`.  Example commands:

```text
/duckduckgo_search query="погода в Москве" limit=20 lang="ru-ru"
```
- `query` – search term (required).
- `limit` – number of results (`auto`, 5‑50 step 5).
- `lang` – language code (`auto`, en‑us, ru‑ru, de‑de, fr‑fr, es‑es).

If `limit` or `lang` are set to `auto`, the plugin uses its default values (5 results, language auto‑detected).

## Technical Details

- **Language**: Python 3, using the Astra Python SDK (`astra-plugin-sdk`).
- **Dependencies**: `requests` for HTTP calls.
- **Entry point**: `src/plugin.py` implements `AstraWebsearchDdg` with the `duckduckgo_search` async method.
- **Configuration schema** (`plugin.toml`):
  ```toml
  [config]
  schema = "{ \"type\": \"object\", \"properties\": { \"default_limit\": { \"type\": \"string\", \"enum\": [\"auto\", \"5\", \"10\", \"15\", \"20\", \"25\", \"30\", \"35\", \"40\", \"45\", \"50\"], \"default\": \"auto\", \"title\": \"Limit of results\" }, \"default_lang\": { \"type\": \"string\", \"enum\": [\"auto\", \"en-us\", \"ru-ru\", \"de-de\", \"fr-fr\", \"es-es\"], \"default\": \"auto\", \"title\": \"Language\" } } }"
  ```
- **Permissions**: none beyond the default (the plugin only calls external HTTP endpoints).

## Repository layout

```
astra-websearch-ddg/
│   plugin.toml          # manifest, config schema, metadata
│   README.md            # this file
│   requirements.txt    # Python dependencies
│
└───src/
        plugin.py       # implementation of the duckduckgo_search tool
```

## Why use this plugin?

- **Instant information** – provides up‑to‑date search results inside an Astra conversation.
- **Customizable output** – users can control how many results they receive and in which language.
- **Privacy aware** – all requests go directly to DuckDuckGo; no data is stored locally.
- **Easy to extend** – the code is compact and can be expanded with additional parameters (e.g., safe‑search, region).

---

*If you need to modify the plugin, edit `src/plugin.py` and run `astra-plugin check .` and `astra-plugin test .` to validate the changes.*
