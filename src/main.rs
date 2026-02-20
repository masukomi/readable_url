/* Downloads an URL, runs it through Readability,
 * optionally converts it to markdown,
 * and sends the results to standard out.
 * */

use clap::Parser;
use readability::extractor;
use readable_url::markdown;

#[derive(Parser)]
#[command(
    name = "readable_url",
    about = "Simplify URL content with Readability & optionally convert to Markdown."
)]
struct Args {
    /// Output as markdown instead of html
    #[arg(short = 'm', long = "markdown")]
    markdown: bool,

    /// The URL to convert
    url: String,
}

// example url:
// https://daringfireball.net/linked/2020/02/25/ipados-file-defaults-snell

// TODO: support local paths
fn main() {
    let args = Args::parse();

    let html_content = extract_html_content(&args.url);
    if args.markdown {
        let markdown_content = extract_markdown_content(html_content);
        println!("{}", markdown_content);
    } else {
        println!("{}", html_content);
    }
}

fn extract_html_content(raw_url: &str) -> String {
    match extractor::scrape(raw_url) {
        Ok(product) => product.content,
        Err(error) => format!("ERROR: {}", error),
    }
}

fn extract_markdown_content(readable_html: String) -> String {
    markdown::convert_string(&readable_html)
}
