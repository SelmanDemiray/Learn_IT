use pulldown_cmark::{Parser, Options, html};
use std::collections::HashMap;
use tera::{Result, Value, from_value};
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

/// Function for rendering markdown to HTML in templates
pub fn markdown_to_html(args: &HashMap<String, Value>) -> Result<Value> {
    let markdown = match args.get("content") {
        Some(val) => from_value::<String>(val.clone())?,
        None => return Err("Missing content argument".into()),
    };

    let html_output = convert_markdown_to_html(&markdown);
    
    Ok(Value::String(html_output))
}

fn convert_markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(&markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Process code blocks with syntax highlighting
    process_code_blocks(&html_output)
}

fn process_code_blocks(html: &str) -> String {
    // A basic implementation that could be expanded for full syntax highlighting
    // In a real implementation, you'd want to use an HTML parser and syntect for full highlighting
    
    let _syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();
    let _theme = &theme_set.themes["base16-ocean.dark"];
    
    // This is a simplified implementation
    html.to_string()
}
