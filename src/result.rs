use crate::file_data::FileData;
use std::collections::HashMap;

/// Struct representing the JSON result containing extracted data.
///
/// This struct represents the JSON result containing information about the extracted data,
/// including a padron identifier, a vector of site data (`FileData`), tag counts,
/// and totals of chatty sites and tags.
///
#[derive(Debug, PartialEq)]
pub struct ResultJson {
    pub padron: String,
    pub sites: Vec<FileData>,
    pub tags: HashMap<String, (usize, usize)>,
    pub totals: Totals,
}

/// Struct representing totals of chatty sites and tags.
///
/// This struct represents the totals of chatty sites and tags, which are part of the
/// overall result containing extracted data.
///
#[derive(Debug, PartialEq)]
pub struct Totals {
    pub chatty_sites: Vec<String>,
    pub chatty_tags: Vec<String>,
}

impl ResultJson {
    /// Print the JSON representation of the result.
    ///
    /// This function prints the JSON representation of the `ResultJson` struct to the standard output.
    /// It formats the data in a structured way, including padron identifier, site data, tag counts,
    /// and totals of chatty sites and tags.
    ///
    pub fn print(&self) {
        println!("{{");
        println!("  \"padron\": \"{}\",", self.padron);
        println!("  \"sites\": {{");
        for (site_index, site) in self.sites.iter().enumerate() {
            println!("    \"{}\": {{", site.filename);
            println!("      \"questions\": {},", site.total_line_count);
            println!("      \"words\": {},", site.total_word_count);
            println!("      \"tags\": {{");
            for (tag_index, (tag, counts)) in site.tag_counts.iter().enumerate() {
                println!("        \"{}\": {{", tag);
                println!("          \"questions\": {},", counts.0);
                println!("          \"words\": {}", counts.1);
                if tag_index < site.tag_counts.len() - 1 {
                    println!("        }},");
                } else {
                    println!("        }}");
                }
            }
            println!("      }},");
            println!("      \"chatty_tags\": {:?}", site.top_tags);
            if site_index < self.sites.len() - 1 {
                println!("    }},");
            } else {
                println!("    }}");
            }
        }

        println!("  }},");
        println!("  \"tags\": {{");
        for (index, (tag, counts)) in self.tags.iter().enumerate() {
            println!("    \"{}\": {{", tag);
            println!("      \"questions\": {},", counts.0);
            println!("      \"words\": {}", counts.1);
            if index < self.tags.len() - 1 {
                println!("    }},");
            } else {
                println!("    }}");
            }
        }
        println!("  }},");
        println!("  \"totals\": {{");
        println!("    \"chatty_sites\": {:?},", self.totals.chatty_sites);
        println!("    \"chatty_tags\": {:?}", self.totals.chatty_tags);
        println!("  }}");
        println!("}}");
    }
}
