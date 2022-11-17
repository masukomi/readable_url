/* Downloads an URL, runs it through Readability,
 * optionally converts it to markdown,
 * and sends the results to standard out.
 * */

extern crate readable_url;
extern crate readability;

use std::env;
use std::str;

use readability::extractor;
use readable_url::markdown;

fn help() {
	println!("USAGE: readable_url <-m|-h> <url/to/convert>
       -m output markdown
       -h output html

       Downloads the specified url, runs it through
       the readability algorithm, optionally converts to markdown
       and prints the result to standard out.");
}



// example url:
// https://daringfireball.net/linked/2020/02/25/ipados-file-defaults-snell

// TODO: support local paths
fn main()  {
	let args: Vec<String> = env::args().collect();

	match args.len() {
		3 => {
			let raw_flag = &args[1];
			let markdown = if raw_flag == "-m" { true } else { false };
			let raw_url = &args[2];
			let html_content = extract_html_content(&raw_url);
			if markdown {
				let markdown_content = extract_markdown_content(html_content);
				println!("{}", markdown_content);
			} else {
				println!("{}", html_content);
			}
		},
		_ => {
			help();
		}
	}
}

fn extract_html_content(raw_url: &str) -> String {
	match extractor::scrape(raw_url) {
      Ok(product) => {
				  return String::from(product.content);
      },
    Err(error) => {return format!("ERROR: {}", error);},
  }
}

fn extract_markdown_content(readable_html: String) -> String {
	return markdown::convert_string(&readable_html);
}
