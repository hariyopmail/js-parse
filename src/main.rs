use clap::{App, Arg};
use colored::*;
use json;
use std::fs;
use std::io::prelude::*;
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
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true),
        )
        .arg(Arg::with_name("all").long("all"))
        .arg(Arg::with_name("subdomains").long("subdomains"))
        .arg(Arg::with_name("endpoints").long("endpoints"))
        .arg(Arg::with_name("parameters").long("parameters"))
        .arg(Arg::with_name("headers").long("headers"))
        .arg(Arg::with_name("keys").long("keys"))
        .arg(Arg::with_name("verbose").short("v").long("verbose"))
        .get_matches();

    let input = args.value_of("input").unwrap();
    let domain = args.value_of("domain").unwrap();
    let output = args.is_present("output");
    let verbosity = args.is_present("verbose");
    let mut all = args.is_present("all");

    if !args.is_present("subdomains")
        && !args.is_present("endpoints")
        && !args.is_present("parameters")
        && !args.is_present("headers")
        && !args.is_present("keys")
    {
        all = true
    }

    let md = match fs::metadata(input) {
        Ok(md) => md,
        Err(err) => {
            eprintln!("{} {}", "error:".bright_red().bold(), err);
            process::exit(-1);
        }
    };
    let mut files = Vec::new();

    if md.is_dir() {
        let dir_files = match fs::read_dir(input) {
            Ok(dir_files) => dir_files,
            Err(err) => {
                eprintln!("{} {}", "error:".bright_red().bold(), err);
                process::exit(-1);
            }
        };
        for f in dir_files {
            let f = f.unwrap().path().to_str().unwrap().to_string();
            files.push(f);
        }
    } else {
        files.push(input.to_string());
    }

    let mut subdomains: Vec<String> = Vec::with_capacity(0xfff);
    let mut endpoints: Vec<String> = Vec::with_capacity(0xfff);
    let mut parameters: Vec<String> = Vec::with_capacity(0xfff);
    let mut headers: Vec<String> = Vec::with_capacity(0xff);
    let mut api_keys: Vec<String> = Vec::with_capacity(0xff);

    for file in files {
        let file = file.as_str();
        let md = fs::metadata(file).unwrap();

        if !md.is_file() {
            continue;
        }

        if verbosity {
            println!("{} scanning {}...", "info:".bright_blue().bold(), file);
        }

        let mut f = match fs::File::open(file) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("{} {}", "error:".bright_red().bold(), err);
                continue;
            }
        };

        let mut buf = vec![];
        match f.read_to_end(&mut buf) {
            Ok(_ok) => _ok,
            Err(err) => {
                eprintln!("{} {}", "error:".bright_red().bold(), err);
                continue;
            }
        };
        let content = String::from_utf8_lossy(&buf);

        if args.is_present("subdomains") || all {
            let subs: Vec<String> = helpers::find_subdomains(&domain, &content);

            for sub in subs {
                if !subdomains.contains(&sub) {
                    subdomains.push(sub);
                }
            }
        }

        if args.is_present("endpoints") || all {
            let endps: Vec<String> = helpers::find_endpoints(&domain, &content);

            for endp in endps {
                if !endpoints.contains(&endp) {
                    endpoints.push(endp);
                }
            }
        }

        if args.is_present("parameters") || all {
            let params: Vec<String> = helpers::find_parameters(&content);

            for param in params {
                if !parameters.contains(&param) {
                    parameters.push(param);
                }
            }
        }

        if args.is_present("headers") || all {
            let hs: Vec<String> = helpers::find_header(&content);

            for h in hs {
                if !headers.contains(&h) {
                    headers.push(h);
                }
            }
        }

        if args.is_present("keys") || all {
            let keys: Vec<String> = helpers::find_api_keys(&content);

            for key in keys {
                if !api_keys.contains(&key) {
                    api_keys.push(key);
                }
            }
        }
    }

    if args.is_present("subdomains") || all {
        if all {
            println!("{}:", "Subdomains".bright_green().bold());
        }
        for sub in &subdomains {
            if all {
                println!("\t{}", sub);
            } else {
                println!("{}", sub);
            }
        }
    }

    if all {
        println!();
    }

    if args.is_present("endpoints") || all {
        if all {
            println!("{}:", "Endpoints".bright_green().bold());
        }
        for endpoint in &endpoints {
            if all {
                println!("\t{}", endpoint);
            } else {
                println!("{}", endpoint);
            }
        }
    }

    if all {
        println!();
    }

    if args.is_present("parameters") || all {
        if all {
            println!("{}:", "Parameters".bright_green().bold());
        }
        for param in &parameters {
            if all {
                println!("\t{}", param);
            } else {
                println!("{}", param);
            }
        }
    }

    if all {
        println!();
    }

    if args.is_present("headers") || all {
        if all {
            println!("{}:", "Headers".bright_green().bold());
        }
        for header in &headers {
            if all {
                println!("\t{}", header);
            } else {
                println!("{}", header);
            }
        }
    }

    if all {
        println!();
    }

    if args.is_present("keys") || all {
        if all {
            println!("{}:", "API Keys".bright_green().bold());
        }
        for key in &api_keys {
            let values: Vec<&str> = key.split("|").collect();
            if all {
                println!("\t{}: {}", values[0].bright_cyan().bold(), values[1]);
            } else {
                println!("{}: {}", values[0].bright_cyan().bold(), values[1]);
            }
        }
    }

    if output {
        let output_file = args.value_of("output").unwrap();

        let mut output = json::JsonValue::new_object();

        output["subdomains"] = subdomains.into();
        output["endpoints"] = endpoints.into();
        output["parameter"] = parameters.into();
        output["headers"] = headers.into();

        for key in &api_keys {
            let values: Vec<&str> = key.split("|").collect();
            output["keys"][values[0]] = values[1].into();
        }

        let mut file = fs::File::create(output_file).unwrap();

        match file.write(output.dump().as_bytes()) {
            Ok(_ok) => println!(
                "{}: {} {}",
                "info".bright_blue().bold(),
                "saved results in",
                output_file
            ),
            Err(err) => eprintln!("{} {}", "error:".bright_red().bold(), err),
        };
    }
}
