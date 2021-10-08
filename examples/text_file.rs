#[macro_use]
extern crate log;

use anyhow::Result;
use fclean::exclusion::Exclusion;

fn main() -> Result<()> {
    env_logger::init();
    
    let exclusions = vec![Exclusion::Empty];

    info!("Cleaning file: text_file.txt");

    let input = include_str!("text_file.txt");
    let mut output_lines: Vec<String> = vec![];

    for line_string in input.lines() {
        let exclude = exclusions.iter().any(|e| e.check(line_string));
        let ex_include = if exclude { "✗"} else { "✓" };

        debug!("{} {}", ex_include, &line_string);

        if !exclude {
            output_lines.push(line_string.to_owned())
        }
    }

    for line in output_lines.iter() {
        println!("{}", line)
    }

    info!("Line count: {}", output_lines.len());

    Ok(())
}