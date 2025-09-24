use actix_web::HttpResponse;
use crate::services::CsvService;

pub async fn list_datasets_handler() -> HttpResponse {
    let datasets = CsvService::get_available_datasets();

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Available Security Datasets</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            justify-content: center;
            align-items: center;
            margin: 0;
        }}
        .container {{
            background: rgba(255, 255, 255, 0.95);
            border-radius: 20px;
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
            padding: 40px;
            max-width: 600px;
        }}
        h1 {{
            color: #333;
            text-align: center;
            margin-bottom: 30px;
        }}
        .dataset-list {{
            list-style: none;
            padding: 0;
        }}
        .dataset-item {{
            background: white;
            margin: 10px 0;
            padding: 15px;
            border-radius: 10px;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
            transition: transform 0.3s;
        }}
        .dataset-item:hover {{
            transform: translateX(10px);
        }}
        a {{
            color: #667eea;
            text-decoration: none;
            font-weight: 600;
        }}
        a:hover {{
            text-decoration: underline;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>🛡️ Security Dashboard Datasets</h1>
        <ul class="dataset-list">
            {}
        </ul>
    </div>
</body>
</html>"#,
        datasets.iter()
            .map(|(name, path)| format!(
                r#"<li class="dataset-item">
                    <a href="/dashboard?file={}">📊 {}</a>
                    <div style="font-size: 0.85em; color: #666; margin-top: 5px;">{}</div>
                </li>"#,
                path,
                name.replace("_", " ").to_uppercase(),
                path
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );

    HttpResponse::Ok().content_type("text/html").body(html)
}