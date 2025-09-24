use csv::Reader;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;

pub struct CsvService;

impl CsvService {
    pub fn read_csv_data<P: AsRef<Path>>(
        file_path: P,
    ) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut csv_reader = Reader::from_reader(reader);

        let mut data = Vec::new();

        // Read headers first
        let headers = csv_reader.headers()?;
        let headers: Vec<String> = headers.iter().map(|h| h.to_string()).collect();
        data.push(headers);

        // Read data rows
        for result in csv_reader.records() {
            let record = result?;
            let row: Vec<String> = record.iter().map(|field| field.to_string()).collect();
            data.push(row);
        }

        Ok(data)
    }

    pub fn get_available_datasets() -> Vec<(String, String)> {
        let mut datasets = Vec::new();

        // Dynamically scan data/ directory for CSV files
        if let Ok(entries) = fs::read_dir("data") {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if let Some(ext) = path.extension() {
                        if ext == "csv" {
                            if let Some(filename) = path.file_stem() {
                                let name = filename.to_string_lossy().to_string();
                                let path_str = path.to_string_lossy().to_string();
                                datasets.push((name, path_str));
                            }
                        }
                    }
                }
            }
        }

        // Sort datasets alphabetically
        datasets.sort_by(|a, b| a.0.cmp(&b.0));

        datasets
    }

    pub fn analyze_csv_data(data: &[Vec<String>]) -> DataAnalysis {
        let mut analysis = DataAnalysis::default();

        if data.is_empty() {
            return analysis;
        }

        // Get headers
        analysis.headers = data[0].clone();
        analysis.total_rows = data.len() - 1;

        // Find numeric columns and calculate stats
        for col_idx in 0..analysis.headers.len() {
            let mut numeric_values = Vec::new();
            let mut categories = std::collections::HashMap::new();

            for row in &data[1..] {
                if let Some(value) = row.get(col_idx) {
                    // Try to parse as number
                    if let Ok(num) = value.parse::<f64>() {
                        numeric_values.push(num);
                    }
                    // Count categories
                    *categories.entry(value.clone()).or_insert(0) += 1;
                }
            }

            // If column is numeric, calculate stats
            if !numeric_values.is_empty() && numeric_values.len() > data.len() / 2 {
                let sum: f64 = numeric_values.iter().sum();
                let avg = sum / numeric_values.len() as f64;
                let min = numeric_values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                let max = numeric_values
                    .iter()
                    .fold(f64::NEG_INFINITY, |a, &b| a.max(b));

                analysis.numeric_columns.push(ColumnStats {
                    name: analysis.headers[col_idx].clone(),
                    min,
                    max,
                    avg,
                    sum,
                });
            }

            // Store category counts for charts
            // Skip columns with too many unique values (like timestamps, IDs)
            if categories.len() > 1 && categories.len() <= 10 {
                // Only for reasonable number of categories
                // Skip columns that look like timestamps or IDs
                let col_name = analysis.headers[col_idx].to_lowercase();
                if !col_name.contains("timestamp")
                    && !col_name.contains("time")
                    && !col_name.contains("date")
                    && !col_name.contains("_id")
                    && !col_name.contains("id")
                {
                    analysis.categorical_columns.push(CategoryStats {
                        name: analysis.headers[col_idx].clone(),
                        categories,
                    });
                }
            }
        }

        analysis
    }
}

#[derive(Debug, Default, serde::Serialize)]
pub struct DataAnalysis {
    pub headers: Vec<String>,
    pub total_rows: usize,
    pub numeric_columns: Vec<ColumnStats>,
    pub categorical_columns: Vec<CategoryStats>,
}

#[derive(Debug, serde::Serialize)]
pub struct ColumnStats {
    pub name: String,
    pub min: f64,
    pub max: f64,
    pub avg: f64,
    pub sum: f64,
}

#[derive(Debug, serde::Serialize)]
pub struct CategoryStats {
    pub name: String,
    pub categories: std::collections::HashMap<String, i32>,
}
