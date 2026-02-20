# readable_url

A simple command line utility that takes an URL, downloads it, runs it through [the arc90 Redability algorithm](https://github.com/masukomi/arc90-readability), optionally
converts that to Markdown, and sends the results to Standard Out.

-----
Note: While this works, Rust's readability library does a notably worse job of extracting content when compared to the Golang implementation as of Feb 2026. It has a tendency to exclude things it shouldn't. 

I recommend you use the Golang [reader](https://github.com/mrusme/reader) utility instead.

-----

```
USAGE: readable_url <-m|-h> <url/to/convert>
	-m output markdown
	-h output html

	Downloads the specified url, runs it through
	the readability algorithm, optionally converts to markdown
	and prints the result to standard out.");

```
## Installation 

Requires [Rust](https://www.rust-lang.org/) and it's associated `cargo` utility.

```sh
cargo install readable_url
```


## License 
MIT See LICENSE.md
