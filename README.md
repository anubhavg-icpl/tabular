# CSV Dashboard

A lightweight Rust web application for visualizing CSV data in a clean, tabular interface.

## Features
- 🚀 Fast CSV parsing and rendering
- 📊 Clean HTML table visualization
- 🦀 Built with Rust and Actix-Web
- 📝 Type-safe templating with Askama

## Quick Start

```bash
# Clone the repository
git clone https://github.com/yourusername/csv-dashboard.git
cd csv-dashboard

# Build and run
cargo run

# Open in browser
# Navigate to http://127.0.0.1:8080/dashboard
```

## Tech Stack
- **Rust** (2021 Edition)
- **Actix-Web** - High-performance web framework
- **Askama** - Type-safe templates
- **CSV** - Data parsing

## Project Structure
```
csv-dashboard/
├── src/
│   ├── main.rs        # Application server
│   └── data.csv       # Sample data
├── templates/
│   └── dashboard.html # Dashboard template
└── Cargo.toml         # Dependencies
```

## License
MIT

## Contributing
Pull requests welcome!