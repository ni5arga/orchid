# Orchid

A minimalist static site generator for Markdown files written in Rust.

## Features

- Convert Markdown files to HTML
- Minimal, clean themes
- Table, images, mermaid support
- Fast and lightweight
- Simple command-line interface

## Installation

```bash
cargo build --release
```

## Usage

```bash
./target/release/orchid <input_dir> [output_dir] [theme]
```

### Arguments

- `input_dir`: Directory containing markdown files (required)
- `output_dir`: Output directory for generated HTML files (default: `./output`)
- `theme`: Theme name - `minimal` or `dark` (default: `minimal`)

### Examples

```bash
# Convert all markdown files in ./content to ./output with minimal theme
./target/release/orchid ./content

# Specify output directory
./target/release/orchid ./content ./dist

# Use dark theme
./target/release/orchid ./content ./output dark
```

## Themes

- **minimal**: Clean, light theme with modern typography
- **dark**: Dark theme with high contrast

## How It Works

1. Scans the input directory for `.md` files
2. Converts each markdown file to HTML
3. Wraps the content in a minimal HTML template with the selected theme
4. Extracts the title from the first `# Heading` or uses the filename
5. Outputs HTML files with the same name as the markdown files

## Example

Given a file `content/hello.md`:

```markdown
# Hello World

This is a **markdown** file.
```

Running `./target/release/orchid ./content` will generate `output/hello.html`.

