# DuckDuckGo Web Search – Astra Plugin

[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/LilKALINOV/astra-websearch-ddg/releases)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Build Status](https://github.com/LilKALINOV/astra-websearch-ddg/actions/workflows/release.yml/badge.svg)](https://github.com/LilKALINOV/astra-websearch-ddg/actions/workflows/release.yml)
[![Astra Plugin](https://img.shields.io/badge/Astra-Plugin-purple)](https://github.com/mihailinl/AstraPlugins)

**Real‑time web search** directly inside your Astra AI assistant – powered by DuckDuckGo, without any built‑in limits.

---

![icon](icon.svg)

---

## 📖 Table of Contents

- [Features](#-features)
- [Why This Plugin?](#-why-this-plugin)
- [How It Works](#-how-it-works)
- [Requirements](#-requirements)
- [Installation](#-installation)
- [License](#-license)
- [Author](#-author)

---

## ✨ Features

- **Live search** – Fetch up‑to‑date results from DuckDuckGo's HTML search endpoint.
- **Configurable result count** – Choose between auto (default 5) or any step‑of‑5 value from 5 to 50.
- **Language support** – Pick from auto (matches your Astra UI language) or explicit codes: en, ru, de, fr, es.
- **Instant Answer fallback** – If the HTML search returns no results, the plugin automatically falls back to DuckDuckGo's Instant Answer API (e.g. for definitions, calculations).
- **Built‑in localisation** – Error and status messages are provided in English, Russian, and Ukrainian.
- **Tool‑only design** – Does not add any panels or UI elements; it works purely as a callable tool.
- **No usage limits** – Unlike Astra's built‑in search (which is restricted in the free version), this plugin gives you unlimited searches via DuckDuckGo.

---

## 🤔 Why This Plugin?

- Unlimited search requests (no daily quotas).
- Full control over result count and language.
- Access to the same high‑quality results that DuckDuckGo provides.
- A fallback to Instant Answers for quick facts.

If you are a free‑tier Astra user, this plugin is essential to unlock the full potential of web‑augmented conversations.

---

## 🔄 How It Works

Here’s a step‑by‑step breakdown of the search flow:

1. **User calls the tool** – via chat command /duckduckgo_search with a query, optional limit, and lang.
2. **Build HTTP request** – the plugin constructs a GET request to DuckDuckGo's HTML endpoint (`https://duckduckgo.com/html/`) with the query and language parameters.
3. **Fetch the page** – using the requests library (simple GET).
4. **Parse HTML** – with BeautifulSoup, extract titles and URLs from the result list.
5. **Fallback if empty** – if no results are found, the plugin queries DuckDuckGo's Instant Answer API (JSON) to return a definition‑style answer.
6. **Return results** – a newline‑separated list of Title – URL strings (or a single definition) is sent back to Astra for display in the chat.

---

## 📡 Requirements

| Resource | Purpose |
|----------|---------|
| **Outbound network access** to `duckduckgo.com:443` | Required to contact DuckDuckGo's public search endpoint. All traffic originates from the plugin process; Astra never sees the query payload or response. |
| **Python 3.8+** | The plugin is written in Python and runs within Astra's plugin runtime. |
| **Astra Desktop Assistant** (v0.8.0 or later) | The plugin targets the Astra plugin SDK. |

No additional host‑side permissions are needed – the plugin runs with the default sandboxed capabilities (`tools = true`).

---

## 📦 Installation

### From the Astra Catalogue (Recommended)

1. Open Astra → **Plugins** → **Browse**.
2. Search for "DuckDuckGo Web Search".
3. Click **Install** – the plugin will be automatically added and ready to use.

### Manual / Development Installation

1. **Enable unsigned plugins** in Astra:  
   Settings → Privacy → **Allow unsigned plugins** (toggle on).
2. Clone this repository:
   ```bash
   git clone https://github.com/LilKALINOV/astra-websearch-ddg.git
   cd astra-websearch-ddg
   cargo build --release
   astra-plugin build
---

   ## 📄 License

This plugin is released under the **MIT License**.  See LICENSE for the full text.

---


## 👤 Author

**Lil KALINOV**  
GitHub: https://github.com/LilKALINOV  
Discord: bass_kalinov

---

## 🙏 Acknowledgements

- DuckDuckGo for the public search endpoint.

Feel free to open issues or submit pull requests if you find bugs or have ideas for improvements.
