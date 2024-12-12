# CrawlCLI

A command-line interface for crawling websites and converting them to Markdown format with token counting capabilities.

## Features

- Crawl any website and convert content to Markdown
- Automatically copy output to clipboard
- Count tokens using OpenAI's cl100k tokenizer (used by text-embedding-ada-002)
- Progress indicator with colorful output
- Flexible output options (file or clipboard-only)

## Prerequisites

- Rust and Cargo (install from [rust-lang.org](https://rust-lang.org))
- Ubuntu/Debian dependencies:
  ```bash
  sudo apt-get install libxcb-shape0-dev libxcb-xfixes0-dev
  ```

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/FirecrawlCLI.git
   cd FirecrawlCLI
   ```

2. Build and install globally:
   ```bash
   cargo install --path .
   ```

The CLI will be installed to `~/.cargo/bin/firecrawl`. Make sure `~/.cargo/bin` is in your PATH.

## Usage

```bash
# Basic usage
firecrawl --url https://example.com

# Specify custom output file
firecrawl -u https://example.com -o custom_output.md

# Copy to clipboard only (no file output)
firecrawl -u https://example.com --clipboard-only

# Set custom page limit (default: 50)
firecrawl -u https://example.com -l 100

# Using API key from environment variable
export FIRECRAWL_API_KEY=your_api_key
firecrawl -u https://example.com

# Or provide API key directly
firecrawl -u https://example.com -a your_api_key
```

## Options

- `-u, --url <URL>`: The URL to crawl (required)
- `-o, --output <FILE>`: Output file path (optional, defaults to domain name)
- `-l, --limit <NUMBER>`: Maximum number of pages to crawl (default: 50)
- `-a, --api-key <KEY>`: Your Firecrawl API key (can also use FIRECRAWL_API_KEY env var)
- `-c, --clipboard-only`: Only copy to clipboard, don't create file
- `-h, --help`: Show help information
- `-V, --version`: Show version information

## Output

The program provides colorful output with:

- Progress spinner while crawling
- Success/error messages in color
- Token count for the generated content
- Clipboard confirmation

## License

[Add your chosen license here]
