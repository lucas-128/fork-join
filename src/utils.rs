use serde_json::Value;

/// Count the total number of words in a collection of text values.
///
/// This function takes a slice of `Value` representing text values and counts
/// the total number of words across all the texts.
///
/// # Arguments
///
/// * `texts`: A slice of `Value` containing text values.
///
/// # Returns
///
/// The total number of words across all texts.
///
pub fn count_words(texts: &[Value]) -> usize {
    texts
        .iter()
        .flat_map(|text| text.as_str())
        .map(|text| text.split_whitespace().count())
        .sum()
}
