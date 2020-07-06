# js-parse

js-parse is inspired by [JSParser](https://github.com/nahamsec/JSParser) and [gitleaks](https://github.com/zricethezav/gitleaks).
js-parse looks through javascript files in a given directory and finds subdomains, relative urls, parameters, custom headers and api keys. The output is formatted
in json, making it easy to integrate into a custom workflow.
There was no other all-in-one tool doing this, so I made this as my first project in rust.

## Installation

```
$ git clone https://github.com/l4yton/js-parse && cd js-parse
$ cargo build --release
```

Binary is located at: `target/release/js-parse`.

Or download the pre-compiled binary from the [releases page](https://github.com/l4yton/js-parse/releases).

## Usage

```
js-parse

USAGE:
    js-parse [FLAGS] [OPTIONS] --domain <domain> --input <input>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    

OPTIONS:
    -d, --domain <domain>    
    -i, --input <input>      
    -o, --output <output>    
```

Examples:
```
$ js-parse -i javascript/ -d site.com -o out.json
```

## Features

- Standard:
    - [X] Subdomains
    - [X] Endpoints
    - [X] Parameter
    - [X] Custom Headers
    - [X] API Keys

- Extra:
    - [ ] 3rd Party Domains
    - [ ] Variable Names
    - [X] Detect Usage of PostMessage 
