use crate::file_data::FileData;
use rayon::prelude::*;
use std::collections::HashMap;

/// Aggregate tag counts from multiple files.
///
/// This function aggregates tag counts from multiple files into a single HashMap.
/// It takes a reference to a vector of `FileData` and returns a HashMap where each
/// key is a tag and the value is a tuple containing the total word count and
/// total line count for that tag across all files.
///
/// # Arguments
///
/// * `files`: A reference to a vector of `FileData` representing the files to aggregate.
///
/// # Returns
///
/// A HashMap containing aggregated tag counts.
///
pub fn aggregate_tag_counts(files: &Vec<FileData>) -> HashMap<String, (usize, usize)> {
    files
        .par_iter()
        .map(|file| {
            file.tag_counts
                .iter()
                .map(|(tag, counts)| {
                    let (word_count, line_count) = counts;
                    (tag.clone(), (*word_count, *line_count))
                })
                .collect::<HashMap<String, (usize, usize)>>()
        })
        .reduce(HashMap::new, |mut acc, map| {
            for (tag, counts) in map {
                let entry = acc.entry(tag).or_insert((0, 0));
                entry.0 += counts.0;
                entry.1 += counts.1;
            }
            acc
        })
}

/// Find the top 10 tags with the highest ratio of words to questions.
///
/// This function calculates the ratio of words to questions for each tag based on the aggregated
/// tag counts and selects the top 10 tags with the highest ratio. The ratio is calculated as the
/// total number of words divided by the total number of questions for each tag.
///
/// # Arguments
///
/// * `aggregated_tag_counts`: A reference to a HashMap containing aggregated tag counts.
///
/// # Returns
///
/// A vector containing the names of the top 10 tags with the highest ratio of words to questions.
///
pub fn top_10_tags_highest_ratio(
    aggregated_tag_counts: &HashMap<String, (usize, usize)>,
) -> Vec<String> {
    let ratios: Vec<(&String, f64)> = aggregated_tag_counts
        .par_iter()
        .map(|(tag, &(questions, words))| {
            let ratio = if questions > 0 {
                words as f64 / questions as f64
            } else {
                0.0
            };
            (tag, ratio)
        })
        .collect();

    let mut sorted_ratios = ratios.clone();
    sorted_ratios.sort_by(|(_, ratio1), (_, ratio2)| {
        match ratio2.partial_cmp(ratio1) {
            Some(order) => order,
            None => std::cmp::Ordering::Equal, // or any other appropriate behavior
        }
    });

    let top_10_tags = sorted_ratios
        .iter()
        .take(10)
        .map(|&(tag, _)| tag.clone())
        .collect::<Vec<String>>();

    top_10_tags
}

/// Calculate the ratio of words to questions for a file.
///
/// This function calculates the ratio of words to questions for a given `FileData` instance.
/// The ratio is calculated as the total number of words divided by the total number of lines
/// in the file. If the total number of lines is zero, the ratio is considered to be zero.
///
/// # Arguments
///
/// * `file_data`: A reference to a `FileData` instance representing the file data.
///
/// # Returns
///
/// The ratio of words to questions as a floating-point number.
///
pub fn words_questions_ratio(file_data: &FileData) -> f64 {
    if file_data.total_line_count > 0 {
        file_data.total_word_count as f64 / file_data.total_line_count as f64
    } else {
        0.0
    }
}

/// Find the top 10 filenames with the highest ratio of words to questions.
///
/// This function calculates the ratio of words to questions for each file and selects the top
/// 10 filenames with the highest ratio. The ratio is calculated as the total number of words
/// divided by the total number of lines in the file. If the total number of lines is zero, the
/// ratio is considered to be zero.
///
/// # Arguments
///
/// * `files_data`: A slice containing references to `FileData` instances representing the file data.
///
/// # Returns
///
/// A vector containing the names of the top 10 filenames with the highest ratio of words to questions.
///
pub fn top_10_filenames_highest_ratio(files_data: &[FileData]) -> Vec<String> {
    let ratios: Vec<(String, f64)> = files_data
        .par_iter()
        .map(|file_data| (file_data.filename.clone(), words_questions_ratio(file_data)))
        .collect();

    let mut sorted_ratios = ratios.clone();
    sorted_ratios.sort_by(|(_, ratio1), (_, ratio2)| {
        ratio2
            .partial_cmp(ratio1)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let top_10_filenames = sorted_ratios
        .iter()
        .take(10)
        .map(|(filename, _)| filename.clone())
        .collect::<Vec<String>>();

    top_10_filenames
}

/// Generate the top tags for each file.
///
/// This function generates the top tags for each file in the given vector of `FileData`.
/// It calculates the ratio of words to questions for each tag in each file's tag counts
/// and selects the top 10 tags with the highest ratio. The ratio is calculated as the
/// total number of words divided by the total number of questions for each tag. If the
/// total number of questions is zero, the ratio is considered to be zero.
///
/// # Arguments
///
/// * `files_data`: A mutable reference to a vector of `FileData` instances representing the file data.
///
pub fn generate_top_tags(files_data: &mut Vec<FileData>) {
    files_data.par_iter_mut().for_each(|file_data| {
        let mut tag_ratios: Vec<(String, f64)> = file_data
            .tag_counts
            .par_iter()
            .map(|(tag, (question_count, tag_word_count))| {
                let ratio = if *question_count > 0 {
                    *tag_word_count as f64 / *question_count as f64
                } else {
                    0.0
                };
                (tag.clone(), ratio)
            })
            .collect();

        tag_ratios.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let top_tags: Vec<String> = tag_ratios
            .into_iter()
            .map(|(tag, _)| tag)
            .take(10)
            .collect();

        file_data.top_tags = top_tags;
    });
}
