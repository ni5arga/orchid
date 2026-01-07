use regex::Regex;

pub fn transform(html: &str) -> String {
    let re = Regex::new(r#"(?s)<pre><code class="language-mermaid">(.*?)</code></pre>"#).unwrap();
    re.replace_all(html, |caps: &regex::Captures| {
        let code = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        format!("<div class=\"mermaid\">{}</div>", code)
    })
    .to_string()
}
