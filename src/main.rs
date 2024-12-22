mod analysis;
mod file_data;
mod result;
mod utils;

use analysis::{
    aggregate_tag_counts, generate_top_tags, top_10_filenames_highest_ratio,
    top_10_tags_highest_ratio,
};
use file_data::process_files;
use result::ResultJson;
use std::env;
use std::time::Instant;

use crate::result::Totals;

/// Entry point of the program.
///
/// This function initializes the program, processes files in a directory,
/// generates statistics, and prints the results.
///
/// # Arguments
///
/// * `args`: Command-line arguments passed to the program.
///
fn main() {
    let args: Vec<String> = env::args().collect();

    let num_threads = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or_else(|_| {
            eprintln!("Invalid thread count");
            std::process::exit(1);
        })
    } else {
        1
    };

    // Set the number of threads.
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap_or_else(|err| {
            eprintln!("Failed to set Rayon thread pool size: {}", err);
            std::process::exit(1);
        });

    let start_time = Instant::now();

    let directory = "data";

    let files = std::fs::read_dir(directory)
        .unwrap_or_else(|err| {
            eprintln!("Failed to open directory: {}", err);
            std::process::exit(1);
        })
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap_or_else(|err| {
            eprintln!("Failed to collect file paths: {}", err);
            std::process::exit(1);
        });

    let mut files_data = process_files(files);

    generate_top_tags(&mut files_data);

    let aggregated_tag_counts = aggregate_tag_counts(&files_data);
    let top_10_tags: Vec<String> = top_10_tags_highest_ratio(&aggregated_tag_counts);
    let top_10_filenames = top_10_filenames_highest_ratio(&files_data);

    // Create Result
    let result = ResultJson {
        padron: "102676".to_string(),
        sites: files_data,
        tags: aggregated_tag_counts,
        totals: Totals {
            chatty_sites: top_10_filenames,
            chatty_tags: top_10_tags,
        },
    };

    // Print Result
    result.print();

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    eprintln!("Time taken: {:?}", elapsed_time);
}
