/* Archives an URL

 requirements:
 crates!
 html-> readibilty library: https://github.com/kumabook/readability
	217 dependencies!!!

 Combine this with html2runes
 to convert an url to markdown
 https://gitlab.com/spacecowboy/html2runes
*/


use std::env;
use std::str;
extern crate readability;
use readability::extractor;

fn help() {
	println!("USAGE: url_to_readability <url/to/convert>
       Downloads the specified url, runs it throug
       the readability algorithm, and prints to standard out.");
}



// example url:
// https://daringfireball.net/linked/2020/02/25/ipados-file-defaults-snell

// TODO: support local paths
fn main()  {
	let args: Vec<String> = env::args().collect();

	match args.len() {
		2 => {
			let raw_url = &args[1];
			extract_content(raw_url);
		},
		_ => {
			help();
		}
	}
}

fn extract_content(raw_url: &str) {
	match extractor::scrape(raw_url) {
      Ok(product) => {
          println!("{}", product.content);
      },
      Err(_) => println!("error occured"),
  }
}
