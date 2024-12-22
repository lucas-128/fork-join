#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use tp1::{
        aggregate_tag_counts, generate_top_tags, process_files, top_10_filenames_highest_ratio,
        top_10_tags_highest_ratio, FileData, ResultJson, Totals,
    };

    fn are_result_json_equal(result1: &ResultJson, result2: &ResultJson) -> bool {
        result1.padron == result2.padron
            && are_sites_equal(&result1.sites, &result2.sites)
            && result1.tags == result2.tags
            && result1.totals == result2.totals
    }

    fn are_sites_equal(sites1: &Vec<FileData>, sites2: &Vec<FileData>) -> bool {
        if sites1.len() != sites2.len() {
            return false;
        }

        let mut filenames1: Vec<&String> = sites1.iter().map(|site| &site.filename).collect();
        let mut filenames2: Vec<&String> = sites2.iter().map(|site| &site.filename).collect();

        filenames1.sort();
        filenames2.sort();

        filenames1 == filenames2
    }

    #[test]
    fn test01_process_all_directory_files() {
        let mut site1_tag_counts = HashMap::new();
        site1_tag_counts.insert(String::from("tag1"), (1, 9));
        site1_tag_counts.insert(String::from("tag2"), (1, 4));
        site1_tag_counts.insert(String::from("tag3"), (2, 9));

        let site1_top_tags = vec![
            String::from("tag1"),
            String::from("tag3"),
            String::from("tag2"),
        ];

        let site1 = FileData {
            filename: String::from("testfile2.jsonl"),
            total_word_count: 18,
            total_line_count: 3,
            tag_counts: site1_tag_counts,
            top_tags: site1_top_tags,
        };

        let mut site2_tag_counts = HashMap::new();
        site2_tag_counts.insert(String::from("tag1"), (2, 9));
        site2_tag_counts.insert(String::from("tag2"), (2, 11));

        let site2_top_tags = vec![String::from("tag2"), String::from("tag1")];

        let site2 = FileData {
            filename: String::from("testfile.jsonl"),
            total_word_count: 15,
            total_line_count: 3,
            tag_counts: site2_tag_counts,
            top_tags: site2_top_tags,
        };

        let mut all_tags = HashMap::new();
        all_tags.insert(String::from("tag2"), (3, 15));
        all_tags.insert(String::from("tag1"), (3, 18));
        all_tags.insert(String::from("tag3"), (2, 9));

        let result2 = ResultJson {
            padron: String::from("102676"),
            sites: vec![site1, site2],
            tags: all_tags,
            totals: Totals {
                chatty_sites: vec![
                    String::from("testfile2.jsonl"),
                    String::from("testfile.jsonl"),
                ],
                chatty_tags: vec![
                    String::from("tag1"),
                    String::from("tag2"),
                    String::from("tag3"),
                ],
            },
        };

        let directory: &str = "tests/testfiles/2files";
        let files = std::fs::read_dir(directory)
            .expect("Failed to open directory")
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .expect("Failed to collect file paths");

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

        assert_eq!(true, are_result_json_equal(&result, &result2));
    }
}
