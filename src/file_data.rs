use std::collections::HashMap;

use crate::utils::count_words;
use rayon::prelude::*;
use serde_json::Value;

/// Struct representing data extracted from a file.
///
/// This struct contains information about a file, including its filename,
/// total word count, total line count, tag counts, and top tags.
///
#[derive(Debug, PartialEq)]
pub struct FileData {
    pub filename: String,
    pub total_word_count: usize,
    pub total_line_count: usize,
    pub tag_counts: HashMap<String, (usize, usize)>,
    pub top_tags: Vec<String>,
}

/// Process files to extract data using the fork-join model.
///
/// This function processes a collection of file paths in parallel using the fork-join model,
/// filtering out files with the ".jsonl" extension, reading their contents, and extracting
/// information such as total word count, total line count, and tag counts from the JSONL format.
///
/// # Arguments
///
/// * `files`: A vector of `PathBuf` representing the paths to the files to be processed.
///
/// # Returns
///
/// A vector of `FileData` containing the extracted data from each processed file.
///
pub fn process_files(files: Vec<std::path::PathBuf>) -> Vec<FileData> {
    files
        .into_par_iter()
        .filter_map(|file_path| {
            if let Some(extension) = file_path.extension() {
                if extension == "jsonl" {
                    if let Some(file_name) = file_path.file_name() {
                        if let Some(file_name) = file_name.to_str() {
                            let file_content =
                                std::fs::read_to_string(&file_path).unwrap_or_else(|err| {
                                    eprintln!("Failed to read file: {}", err);
                                    std::process::exit(1);
                                });

                            let (total_word_count, total_line_count, tag_counts) = file_content
                                .lines()
                                .collect::<Vec<_>>()
                                .par_iter()
                                .fold(
                                    || (0, 0, HashMap::new()),
                                    |(word_count, line_count, mut tag_counts), line| {
                                        let v: Value = serde_json::from_str(line)
    .unwrap_or_else(|err| {
        eprintln!("Failed to parse JSON: {}", err);
        std::process::exit(1);
    });

                                        let texts_vec = vec![];
                                        let tags_vec = vec![];

                                        let texts = v
                                            .get("texts")
                                            .and_then(|texts| texts.as_array())
                                            .unwrap_or(&texts_vec);
                                        let tags = v
                                            .get("tags")
                                            .and_then(|tags| tags.as_array())
                                            .unwrap_or(&tags_vec);

                                        for tag in tags {
                                            if let Some(tag) = tag.as_str() {
                                                let tag_entry =
                                                    tag_counts
                                                        .entry(tag.to_string())
                                                        .or_insert((0, 0));
                                                tag_entry.0 += 1;
                                                tag_entry.1 += count_words(texts);
                                            }
                                        }

                                        (
                                            word_count + count_words(texts),
                                            line_count + 1,
                                            tag_counts,
                                        )
                                    },
                                )
                                .reduce(
                                    || (0, 0, HashMap::new()),
                                    |(word_count1, line_count1, mut tag_counts1),
                                     (word_count2, line_count2, tag_counts2)| {
                                        for (tag, (line_count, word_count)) in tag_counts2 {
                                            let tag_entry =
                                                tag_counts1.entry(tag).or_insert((0, 0));
                                            tag_entry.0 += line_count;
                                            tag_entry.1 += word_count;
                                        }
                                        (
                                            word_count1 + word_count2,
                                            line_count1 + line_count2,
                                            tag_counts1,
                                        )
                                    },
                                );

                            return Some(FileData {
                                filename: file_name.to_string(),
                                total_word_count,
                                total_line_count,
                                tag_counts,
                                top_tags: Vec::new(),
                            });
                        }
                    }
                }
            }
            None
        })
        .collect()
}
