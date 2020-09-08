use clap::{App, Arg};
use colored::*;
use std::fs;
use std::process;

mod helpers;

fn main() {
    let args = App::new("js-parse")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("domain")
                .short("d")
                .long("domain")
                .takes_value(true)
                .required(true),
        )
        .arg(Arg::with_name("subdomains").long("subdomains"))
        .arg(Arg::with_name("endpoints").long("endpoints"))
        .arg(Arg::with_name("parameters").long("parameters"))
        .arg(Arg::with_name("headers").long("headers"))
        .arg(Arg::with_name("keys").long("keys"))
        .arg(Arg::with_name("verbose").short("v").long("verbose"))
        .get_matches();

    if !args.is_present("subdomains")
        && !args.is_present("endpoints")
        && !args.is_present("parameters")
        && !args.is_present("headers")
        && !args.is_present("keys")
    {
        eprintln!(
            "{} {}",
            "error:".bright_red().bold(),
            "Please specify at least one option of what to look for. See -h for help."
        );
    }

    let input = args.value_of("input").unwrap();
    let domain = args.value_of("domain").unwrap();
    let verbosity = args.is_present("verbose");

    let files = match fs::read_dir(input) {
        Ok(files) => files,
        Err(err) => {
            eprintln!("{} {}", "error:".bright_red().bold(), err);
            process::exit(-1);
        }
    };

    let mut subdomains: Vec<String> = Vec::with_capacity(0xfff);
    let mut endpoints: Vec<String> = Vec::with_capacity(0xfff);
    let mut parameters: Vec<String> = Vec::with_capacity(0xfff);
    let mut headers: Vec<String> = Vec::with_capacity(0xff);
    let mut api_keys: Vec<String> = Vec::with_capacity(0xff);

    for file in files {
        let file = file.unwrap();
        let md = fs::metadata(file.path()).unwrap();

        if !md.is_file() {
            continue;
        }

        if verbosity {
            println!(
                "{} scanning {}...",
                "info:".bright_blue().bold(),
                file.path().to_string_lossy()
            );
        }

        let content = match fs::read_to_string(file.path()) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("{} {}", "error:".bright_red().bold(), err);
                continue;
            }
        };

        if args.is_present("subdomains") {
            let subs: Vec<String> = helpers::find_subdomains(&domain, &content);

            for sub in subs {
                if !subdomains.contains(&sub) {
                    subdomains.push(sub);
                }
            }
        }

        if args.is_present("endpoints") {
            let endps: Vec<String> = helpers::find_endpoints(&domain, &content);

            for endp in endps {
                if !endpoints.contains(&endp) {
                    endpoints.push(endp);
                }
            }
        }

        if args.is_present("parameters") {
            let params: Vec<String> = helpers::find_parameters(&content);

            for param in params {
                if !parameters.contains(&param) {
                    parameters.push(param);
                }
            }
        }

        if args.is_present("headers") {
            let hs: Vec<String> = helpers::find_header(&content);

            for h in hs {
                if !headers.contains(&h) {
                    headers.push(h);
                }
            }
        }

        if args.is_present("keys") {
            let keys: Vec<String> = helpers::find_api_keys(&content);

            for key in keys {
                if !api_keys.contains(&key) {
                    api_keys.push(key);
                }
            }
        }
    }

    if args.is_present("subdomains") {
        for sub in &subdomains {
            println!("{}", sub);
        }
    }

    if args.is_present("endpoints") {
        for endpoint in &endpoints {
            println!("{}", endpoint);
        }
    }

    if args.is_present("parameters") {
        for param in &parameters {
            println!("{}", param);
        }
    }

    if args.is_present("headers") {
        for header in &headers {
            println!("{}", header);
        }
    }

    if args.is_present("keys") {
        for key in &api_keys {
            let values: Vec<&str> = key.split("|").collect();
            println!("{}: {}", values[0].bright_cyan().bold(), values[1]);
        }
    }
}
