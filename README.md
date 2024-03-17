# readable_url

----
### ⚠️ Rust has moved on since this was created and I'm not a rust geek. 

This is a trivially small lib, but i'm not a Rust geek. So, it's dead for the moment. If you feel like making a PR that updates it to work with current Rust, I'll happily merge it and push the update to Cargo. 

----

A simple command line utility that takes an URL, downloads it, runs it through [the arc90 Redability algorithm](https://github.com/masukomi/arc90-readability), optionally
converts that to Markdown, and sends the results to Standard Out.



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
