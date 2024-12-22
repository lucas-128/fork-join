#[cfg(test)]
mod tests {

    use std::collections::HashMap;
    use tp1::{
        aggregate_tag_counts, generate_top_tags, process_files, top_10_filenames_highest_ratio,
        top_10_tags_highest_ratio, FileData,
    };

    // Function to compare two FileData structs
    pub fn compare_file_data(file_data1: &FileData, file_data2: &FileData) -> bool {
        file_data1.filename == file_data2.filename
            && file_data1.total_word_count == file_data2.total_word_count
            && file_data1.total_line_count == file_data2.total_line_count
            && file_data1.tag_counts == file_data2.tag_counts
            && file_data1.top_tags == file_data2.top_tags
    }

    #[test]
    fn test01_process_directory_file() {
        // Set up comparisson object
        let mut tag_counts = HashMap::new();
        tag_counts.insert(String::from("tag2"), (2, 11));
        tag_counts.insert(String::from("tag1"), (2, 9));

        let file_data = FileData {
            filename: String::from("testfile.jsonl"),
            total_word_count: 15,
            total_line_count: 3,
            tag_counts: tag_counts,
            top_tags: Vec::new(),
        };

        let directory: &str = "tests/testfiles/1file";
        let files = std::fs::read_dir(directory)
            .expect("Failed to open directory")
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .expect("Failed to collect file paths");

        let processed_file_data = &process_files(files)[0];

        assert_eq!(true, compare_file_data(processed_file_data, &file_data));
    }

    #[test]
    fn test02_generate_top_tags_for_single_file() {
        // Set up comparisson object
        let mut tag_counts = HashMap::new();
        tag_counts.insert(String::from("tag1"), (2, 9));
        tag_counts.insert(String::from("tag2"), (2, 11));

        let top_tags = vec![String::from("tag2"), String::from("tag1")];

        let file_data = FileData {
            filename: String::from("testfile.jsonl"),
            total_word_count: 15,
            total_line_count: 3,
            tag_counts,
            top_tags,
        };

        let directory: &str = "tests/testfiles/1file";
        let files = std::fs::read_dir(directory)
            .expect("Failed to open directory")
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .expect("Failed to collect file paths");

        let mut processed_file_data = process_files(files);
        generate_top_tags(&mut processed_file_data);
        let final_file_data = &processed_file_data[0];

        assert_eq!(true, compare_file_data(final_file_data, &file_data));
    }

    #[test]
    fn test03_aggregated_tag_counts() {
        // Set up comparisson object
        let mut tag_counts = HashMap::new();
        tag_counts.insert(String::from("tag2"), (2, 11));
        tag_counts.insert(String::from("tag1"), (2, 9));

        let directory: &str = "tests/testfiles/1file";
        let files = std::fs::read_dir(directory)
            .expect("Failed to open directory")
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .expect("Failed to collect file paths");

        let mut processed_file_data = process_files(files);
        generate_top_tags(&mut processed_file_data);
        let aggregated_tag_counts = aggregate_tag_counts(&processed_file_data);

        assert_eq!(true, aggregated_tag_counts == tag_counts);
    }

    #[test]
    fn test04_top_10_tags() {
        // Set up comparisson object
        let top_tags = vec!["tag2".to_string(), "tag1".to_string()];

        let directory: &str = "tests/testfiles/1file";
        let files = std::fs::read_dir(directory)
            .expect("Failed to open directory")
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .expect("Failed to collect file paths");

        let mut processed_file_data = process_files(files);
        generate_top_tags(&mut processed_file_data);
        let aggregated_tag_counts = aggregate_tag_counts(&processed_file_data);
        let top_10_tags = top_10_tags_highest_ratio(&aggregated_tag_counts);

        assert_eq!(true, top_10_tags == top_tags);
    }

    #[test]
    fn test05_top_10_filenames() {
        // Set up comparisson object
        let filenames = vec!["testfile.jsonl".to_string()];

        let directory: &str = "tests/testfiles/1file";
        let files = std::fs::read_dir(directory)
            .expect("Failed to open directory")
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .expect("Failed to collect file paths");

        let mut processed_file_data = process_files(files);
        generate_top_tags(&mut processed_file_data);
        let top_10_filenames = top_10_filenames_highest_ratio(&processed_file_data);

        assert_eq!(true, top_10_filenames == filenames);
    }
}
