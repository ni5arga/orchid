use pulldown_cmark::{html, Options, Parser};

mod mermaid;
mod images;
mod tables;

pub fn render(content: &str) -> String {
    let parser = Parser::new_ext(content, Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let html_output = mermaid::transform(&html_output);
    let html_output = images::transform(&html_output);
    let html_output = tables::transform(&html_output);

    html_output
}

pub fn needs_mermaid(content: &str) -> bool {
    content.contains("```mermaid") || content.contains("language-mermaid")
}
