mod cli;

use clap::Parser;
use humansize::{format_size, DECIMAL};
use humantime::format_duration;
use indexmap::map::IndexMap;
use rayon::prelude::*;
use regex::{Regex, RegexBuilder};
use std::collections::HashMap;
use std::fs::{metadata, read_to_string};
use std::time::Instant;

const LOG_REGEX: &str = r#"^(?<address>\S*)\s+-\s+(?<user>\S*)\s*-\s+\[(?<time>[a-zA-Z0-9/:\s\+]+)\]\s+"(?<request>[^"]*)"\s+(?<status>\S*)\s+(?<bytes_sent>\S*)\s*"(?<referer>[^"]*)"\s+"(?<user_agent>[^"]*)"$"#;

fn main() {
    let args = cli::Arguments::parse_from(wild::args());

    let start_instant = Instant::now();

    let regex: Regex = RegexBuilder::new(LOG_REGEX)
        .crlf(true)
        .multi_line(true)
        .build()
        .unwrap();

    let exclude_regex: Regex = Regex::new(args.exclude.as_str()).unwrap();
    let whitelist_regex: Regex = Regex::new(args.whitelist.as_str()).unwrap();

    let file_count = args.files.len();
    let file_bytes = args.files
        .iter()
        .fold(0u64, |acc, file_path| acc + metadata(file_path).unwrap().len());

    if !args.quiet {
        println!("Processing {} of logs in {} files.", format_size(file_bytes, DECIMAL), file_count);
    }

    let mut frequencies: IndexMap<String, u64> = args.files.par_iter()
        .map(|file_path|
            regex.captures_iter(read_to_string(file_path).unwrap().as_str())
                .fold(HashMap::new(), |mut acc, capture| {
                    let val = capture[args.column.to_string().as_str()].to_owned();
                    let mut include = true;

                    if !args.exclude.is_empty() && exclude_regex.is_match(val.as_str()) {
                        include = false;
                    }

                    if !args.whitelist.is_empty() && !whitelist_regex.is_match(val.as_str()) {
                        include = false;
                    }

                    if include {
                        *acc.entry(val).or_default() += 1;
                    }

                    acc
                })
        ).collect::<Vec<HashMap<String, u64>>>()
        .iter()
        .fold(IndexMap::new(), |mut acc, freqs| {
            for key in freqs.keys() {
                *acc.entry(key.to_owned()).or_default() += freqs.get(key).unwrap();
            }

            acc
        });

    frequencies
        .sort_by(|_a_key, a_val, _b_key, b_val|
            match args.order {
                cli::SortOrder::Asc => a_val.cmp(b_val),
                cli::SortOrder::Desc => b_val.cmp(a_val)
            }
        );

    if args.limit > 0 {
        frequencies
            .truncate(args.limit as usize);
    }

    if !args.quiet {
        println!("Finished processing in {}.", format_duration(start_instant.elapsed()));

        if args.limit > 0 {
            println!("Showing top {} results.", args.limit);
        }

        println!();
    }

    for (key, val) in frequencies.iter() {
        if args.no_count {
            println!("{}", key);
        } else {
            println!("{:10} - {}", val, key);
        }
    }
}
