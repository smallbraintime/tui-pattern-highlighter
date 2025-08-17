# tui-pattern-highlighter

![Version](https://img.shields.io/badge/version-0.3.3-orange.svg)
![Tests](https://img.shields.io/badge/tests-passing-green.svg)
![Docs](https://img.shields.io/badge/docs-passing-green.svg)

a dead simple search pattern highlighter for ratatui

## Example
```rust
use tui_pattern_highlighter::highlight_line;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span, Text},
};

let line = "Hi @buddy";
let pattern = r"@\w+";
let highlight_style = Style::new().bg(Color::Blue);

let expected_line = Line::from(vec![
    Span::from("Hi "),
    Span::from("@buddy").style(Style::new().bg(Color::Blue)),
]);

assert_eq!(highlight_line(line, pattern, highlight_style), expected_line);

let text = "Hi @buddy\n@stranger hello";
let pattern = r"@\w+";
let highlight_style = Style::new().bg(Color::Blue);

let expected_text = Text::from(vec![
    Line::from(vec![
        Span::from("Hi "),
        Span::from("@buddy").style(Style::new().bg(Color::Blue)),
    ]),
    Line::from(vec![
        Span::from("@stranger").style(Style::new().bg(Color::Blue)),
        Span::from(" hello"),
    ]),
]);

assert_eq!(highlight_text(text, pattern, highlight_style), expected_text);
```

