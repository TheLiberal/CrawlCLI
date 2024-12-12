use clap::Parser;
use firecrawl::{
    crawl::{CrawlOptions, CrawlScrapeOptions, CrawlScrapeFormats},
    FirecrawlApp,
    FirecrawlError
};
use std::fs::File;
use std::io::Write;
use tokio;
use clipboard::{ClipboardProvider, ClipboardContext};
use url;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use tiktoken_rs::cl100k_base;

mod token_counter {
    use super::*;

    pub fn count_tokens(text: &str) {
        let bpe = cl100k_base().unwrap();
        let token_count = bpe.encode_with_special_tokens(text).len();

        println!(
            "{}{}{} Token count: {}",
            "[".bold().white(),
            "i".bold().blue(),
            "]".bold().white(),
            token_count.to_string().bold().yellow(),
        );
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The URL to crawl
    #[arg(short, long)]
    url: String,

    /// The output file path
    #[arg(short, long)]
    output: Option<String>,

    /// The maximum number of pages to crawl
    #[arg(short, long, default_value_t = 50)]
    limit: u32,

    /// Your Firecrawl API key
    #[arg(short, long, env = "FIRECRAWL_API_KEY")]
    api_key: String,

    /// Flag to disable outputting to file, just copy to clipboard
    #[arg(short, long, default_value_t = false)]
    clipboard_only: bool,
}

async fn crawl_and_write(args: Args) -> Result<(), FirecrawlError>{
    // Initialize the FirecrawlApp with the API key
    let app = FirecrawlApp::new(&args.api_key).expect("Failed to initialize FirecrawlApp");

    println!("{}", "Starting crawl...".bright_blue().bold());
    
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈")
            .template("{spinner:.blue} {msg}")
            .unwrap()
    );
    spinner.set_message("Crawling website...");
    spinner.enable_steady_tick(Duration::from_millis(120));

    // Configure crawl options
    let crawl_options = CrawlOptions {
        scrape_options: CrawlScrapeOptions {
            formats: vec![ CrawlScrapeFormats::Markdown,  ].into(),
            ..Default::default()
        }.into(),
        limit: args.limit.into(),
        ..Default::default()
    };

    // Perform the crawl
    let crawl_result = app.crawl_url(&args.url, crawl_options).await;

    // Handle crawl result and write to output file
    match crawl_result {
        Ok(data) => {
            spinner.finish_with_message("Crawl completed successfully!".green().to_string());
            let mut markdown_output = String::new();

            for item in data.data {
                if let Some(markdown) = item.markdown {
                    markdown_output.push_str(&markdown);
                    markdown_output.push_str("\n");
                }
            }

            if !args.clipboard_only {
                let output_file_name = match args.output {
                    Some(output) => output,
                    None => {
                        let parsed_url = url::Url::parse(&args.url).expect("Invalid URL");
                        let domain = parsed_url.host_str().expect("Could not get domain");
                        let domain = domain.split('.').next().expect("Could not get domain without extension");
                        format!("{}.md", domain)
                    }
                };

                let mut output_file = File::create(&output_file_name).expect("Failed to create output file");
                writeln!(output_file, "{}", markdown_output).expect("Failed to write to output file");
                println!("{} {}", "✓".green(), format!("Wrote output to {}", &output_file_name).green());
            }

            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(markdown_output.clone()).unwrap();
            if args.clipboard_only {
                println!("{} {}", "✓".green(), "Output copied to clipboard".green());
            } else {
                println!("{} {}", "✓".green(), "Output also copied to clipboard".green());
            }

            // Count tokens after successful operations
            token_counter::count_tokens(&markdown_output);

            Ok(())
        }
        Err(e) => {
            spinner.finish_with_message("Crawl failed!".red().bold().to_string());
            eprintln!("{} {}", "✗".red(), format!("Error: {}", e).red());
            Err(e)
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    if let Err(e) = crawl_and_write(args).await {
       eprintln!("{} {}", "✗".red(), format!("An error occurred: {}", e).red());
       std::process::exit(1);
    }
}