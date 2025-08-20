# ASIMOV Readwise Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Package on Crates.io](https://img.shields.io/crates/v/asimov-readwise-module)](https://crates.io/crates/asimov-readwise-module)
[![Documentation](https://docs.rs/asimov-readwise-module/badge.svg)](https://docs.rs/asimov-readwise-module)

[ASIMOV] module for importing data from [Readwise] and converting it to [JSON-LD] format.

## ✨ Features

- 📚 **Import Highlights**: Fetch and convert Readwise highlights to JSON-LD
- 📖 **Import Books**: Fetch and convert Readwise book lists to JSON-LD
- 🏷️ **Import Tags**: Fetch and convert Readwise tags to JSON-LD
- 🔄 **JSON-LD Output**: Structured data compatible with [KNOW] ontology
- ⚡ **Fast & Reliable**: Built with Rust for performance and safety
- 🚦 **Rate Limit Aware**: Respects Readwise API limits and provides clear error handling

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition) if building from source code
- [Readwise] account with API access
- `READWISE_API_KEY` environment variable

## ⬇️ Installation

### Installation from Package Managers

```bash
# From PyPI (Python)
pip install -U asimov-readwise-module

# From RubyGems (Ruby)
gem install asimov-readwise-module

# From NPM (Node.js)
npm install -g asimov-readwise-module

# From Cargo (Rust)
cargo install asimov-readwise-module
```

## 👉 Examples

### Import Highlights

```bash
# Import all highlights (transparent pagination)
asimov-readwise-importer https://readwise.io/highlights

# Import first 100 highlights only
asimov-readwise-importer https://readwise.io/highlights --page-size 100

# Import specific page
asimov-readwise-importer https://readwise.io/highlights --page 2 --page-size 50
```

### Import Books

```bash
# Import all books (transparent pagination)
asimov-readwise-importer https://readwise.io/books

# Import first 50 books only
asimov-readwise-importer https://readwise.io/books --page-size 50
```

### Import Tags

```bash
asimov-readwise-importer https://readwise.io/tags
```

### Other Commands

```bash
# Show version information
asimov-readwise-importer --version

# Show license information
asimov-readwise-importer --license

# Show help
asimov-readwise-importer --help
```

## ⚙ Configuration

### API Key Setup

Set your Readwise API key as an environment variable:

```bash
export READWISE_API_KEY="your-api-key-here"
```

Or use a `.env` file:

```env
READWISE_API_KEY=your-api-key-here
```

Get your API key from: <https://readwise.io/access_token>

### Rate Limiting

**Heads up**: This module plays nice with Readwise's API limits. If you get HTTP 429 errors (too many requests):

- **Just wait a bit**: The API will throw an error if you're going too fast
- **No auto-retry**: We don't automatically retry failed requests
- **Take it slow**: If you're grabbing lots of data, give it some breathing room

**The limits**: This plugin is limited to 20 requests per minute due to Readwise's API restrictions.

## 👨‍💻 Development

```bash
git clone https://github.com/asimov-modules/asimov-readwise-module.git
cd asimov-readwise-module
cargo test
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/asimov-modules/asimov-readwise-module&text=asimov-readwise-module)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/asimov-modules/asimov-readwise-module&title=asimov-readwise-module)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/asimov-modules/asimov-readwise-module&t=asimov-readwise-module)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/asimov-modules/asimov-readwise-module)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/asimov-modules/asimov-readwise-module)

[ASIMOV]: https://asimov.sh
[ASIMOV CLI]: https://cli.asimov.sh
[JSON-LD]: https://json-ld.org
[KNOW]: https://know.dev
[RDF]: https://www.w3.org/TR/rdf12-primer/
[Readwise]: https://readwise.io
[Rust]: https://rust-lang.org
