use csv::Reader;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct CsvService;

impl CsvService {
    pub fn read_csv_data<P: AsRef<Path>>(file_path: P) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
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
        vec![
            ("security_alerts".to_string(), "data/security_alerts.csv".to_string()),
            ("vulnerabilities".to_string(), "data/vulnerability_scan.csv".to_string()),
            ("threat_intel".to_string(), "data/threat_intelligence.csv".to_string()),
        ]
    }
}