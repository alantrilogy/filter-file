#[macro_use]
extern crate log;

use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

use anyhow::Result;
use clap::Clap;
use env_logger::Builder;
use filter_file::{
    cli,
    exclusion::Exclusion::{Empty, Only, StartsWith},
};

fn main() -> Result<()> {
    let opts = cli::Opts::parse();

    // Log setup
    Builder::new().filter_level(opts.debug).init();

    // Exclusions
    let exclusions = vec![
        Empty,
        Only(vec!['{', '}', '(', ')', ';', '[', ']']),
        StartsWith("//".to_string()),
        StartsWith("///".to_string()),
        StartsWith("//!".to_string()),
    ];

    info!("Reading file: {}", opts.input);
    let input = File::open(&opts.input)?;
    let mut output_lines: Vec<String> = vec![];

    let mut start: usize = 0;
    let mut range: usize = usize::MAX;
    if let Some(opts_from) = opts.from {
        // Include start line
        start = if opts_from == 0 { 0 } else { opts_from - 1 }
    }
    if let Some(opts_to) = opts.to {
        range = opts_to - start
    }

    for line in BufReader::new(input).lines().skip(start).take(range) {
        let line_string = line.unwrap();
        let exclude = exclusions
            .iter()
            .any(|exclusion| exclusion.check(&line_string));
        debug!("Exclude ({}) current line: {}", exclude, &line_string);
        if !exclude {
            output_lines.push(line_string);
        }
    }

    println!("Line count: {}", output_lines.len());

    let output_file =
        File::create(format!("{}_cleaned", &opts.input)).expect("Unable to create file");
    let mut output_file = BufWriter::new(output_file);
    for line in output_lines.iter() {
        writeln!(output_file, "{}", line).expect("Unable to write line to file");
    }

    Ok(())
}
