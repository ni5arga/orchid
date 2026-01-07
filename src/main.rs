use std::fs;
use std::path::Path;
mod markdown;

struct Theme {
    name: String,
    css: String,
}

impl Theme {
    fn minimal() -> Self {
        Theme {
            name: "minimal".to_string(),
            css: include_str!("../themes/minimal.css").to_string(),
        }
    }

    fn dark() -> Self {
        Theme {
            name: "dark".to_string(),
            css: include_str!("../themes/dark.css").to_string(),
        }
    }

    fn get(name: &str) -> Self {
        match name {
            "dark" => Theme::dark(),
            _ => Theme::minimal(),
        }
    }
}

fn markdown_to_html(content: &str) -> String {
    markdown::render(content)
}

fn generate_html_page(markdown_content: &str, title: &str, theme: &Theme) -> String {
    let body_html = markdown_to_html(markdown_content);
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>{}</style>
    <script src="https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.min.js"></script>
    <script>mermaid.initialize({{ startOnLoad: true }});</script>
</head>
<body>
    <div class="container">
        {}
    </div>
</body>
</html>"#,
        title, theme.css, body_html
    )
}

fn process_markdown_file(input_path: &Path, output_dir: &Path, theme: &Theme) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    let file_stem = input_path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("index");
    
    let title = extract_title(&content).unwrap_or_else(|| file_stem.to_string());
    let html = generate_html_page(&content, &title, theme);
    
    let output_path = output_dir.join(format!("{}.html", file_stem));
    fs::write(&output_path, html)?;
    println!("Generated: {}", output_path.display());
    
    Ok(())
}

fn extract_title(content: &str) -> Option<String> {
    content.lines()
        .find(|line| line.starts_with("# "))
        .map(|line| line.trim_start_matches("# ").trim().to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <input_dir> [output_dir] [theme]", args[0]);
        eprintln!("  input_dir:  Directory containing markdown files");
        eprintln!("  output_dir: Output directory (default: ./output)");
        eprintln!("  theme:      Theme name (default: minimal, options: minimal, dark)");
        std::process::exit(1);
    }
    
    let input_dir = Path::new(&args[1]);
    let output_dir = if args.len() > 2 {
        Path::new(&args[2])
    } else {
        Path::new("./output")
    };
    
    let theme_name = args.get(3).map(|s| s.as_str()).unwrap_or("minimal");
    let theme = Theme::get(theme_name);
    
    if !input_dir.exists() || !input_dir.is_dir() {
        eprintln!("Error: Input directory does not exist: {}", input_dir.display());
        std::process::exit(1);
    }
    
    fs::create_dir_all(output_dir)?;
    
    let entries = fs::read_dir(input_dir)?;
    let mut processed = 0;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            process_markdown_file(&path, output_dir, &theme)?;
            processed += 1;
        }
    }
    
    if processed == 0 {
        eprintln!("Warning: No markdown files found in {}", input_dir.display());
    } else {
        println!("Processed {} markdown file(s)", processed);
    }
    
    Ok(())
}

