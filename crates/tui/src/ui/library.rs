// crates/tui/src/ui/library.rs
//! Enhanced library view with detailed information and beautiful formatting

use crate::state::AppState;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Row, Table},
    Frame,
};
use storystream_core::types::book::Book;

/// Renders the enhanced library view with rich information
pub fn render(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
    theme: &crate::theme::Theme,
    books: &[Book],
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Search/filter bar
            Constraint::Min(0),     // Book list
            Constraint::Length(5),  // Selected book details
            Constraint::Length(4),  // Library statistics
        ])
        .split(area);

    render_search_bar(frame, chunks[0], state, theme);
    render_detailed_book_list(frame, chunks[1], state, theme, books);
    render_book_details(frame, chunks[2], state, theme, books);
    render_library_stats(frame, chunks[3], state, theme, books);
}

/// Renders search and filter bar
fn render_search_bar(
    frame: &mut Frame,
    area: Rect,
    _state: &AppState,
    theme: &crate::theme::Theme,
) {
    let text = vec![Line::from(vec![
        Span::styled("🔍 Search: ", theme.accent_style()),
        Span::styled("[Type to search]", theme.text_secondary_style()),
        Span::raw("  |  "),
        Span::styled("Sort: ", theme.accent_style()),
        Span::styled("Title ▼", theme.highlight_style()),
        Span::raw("  |  "),
        Span::styled("Filter: ", theme.accent_style()),
        Span::styled("All", theme.text_style()),
    ])];

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border_color())),
        )
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, area);
}

/// Renders detailed book list with comprehensive information
fn render_detailed_book_list(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
    theme: &crate::theme::Theme,
    books: &[Book],
) {
    if books.is_empty() {
        render_empty_library(frame, area, theme);
        return;
    }

    let items: Vec<ListItem> = books
        .iter()
        .enumerate()
        .map(|(i, book)| {
            let is_selected = i == state.selected_item;
            let style = if is_selected {
                theme.highlight_style().add_modifier(Modifier::BOLD)
            } else {
                theme.text_style()
            };

            // Format duration nicely
            let duration_str = format_duration(book.duration_seconds.unwrap_or(0) as u64);

            // Calculate file size in MB/GB
            let file_size_str = format_file_size(book.file_size_bytes.unwrap_or(0));

            // Progress indicator
            let progress = book.progress_percentage();
            let progress_bar = create_progress_bar(progress, 15);
            let progress_style = if progress >= 100.0 {
                Style::default().fg(Color::Green)
            } else if progress > 0.0 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            // Status icon
            let status_icon = if progress >= 100.0 {
                "✓"
            } else if progress > 0.0 {
                "▶"
            } else {
                "○"
            };

            let line = Line::from(vec![
                Span::styled(
                    format!("{} ", status_icon),
                    if is_selected {
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Green)
                    },
                ),
                Span::styled(
                    format!("{:<45} ", truncate(&book.title, 45)),
                    style.add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("by {:<25} ", truncate(&book.author.clone().unwrap_or_else(|| "Unknown".to_string()), 25)),
                    if is_selected {
                        theme.text_secondary_style().add_modifier(Modifier::BOLD)
                    } else {
                        theme.text_secondary_style()
                    },
                ),
                Span::styled(
                    format!("⏱ {:>10} ", duration_str),
                    Style::default().fg(Color::Cyan),
                ),
                Span::styled(
                    format!("💾 {:>8} ", file_size_str),
                    Style::default().fg(Color::Magenta),
                ),
                Span::styled(progress_bar, progress_style),
                Span::styled(
                    format!(" {:>3}%", progress as u8),
                    if is_selected {
                        progress_style.add_modifier(Modifier::BOLD)
                    } else {
                        progress_style
                    },
                ),
            ]);

            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border_color()))
                .title(format!(
                    "📚 Library - {} Books (↑/↓: Navigate | Enter: Play | /: Search | s: Sort)",
                    books.len()
                )),
        )
        .style(theme.text_style());

    frame.render_widget(list, area);
}

/// Renders detailed information about selected book
fn render_book_details(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
    theme: &crate::theme::Theme,
    books: &[Book],
) {
    if books.is_empty() {
        return;
    }

    let book = if let Some(b) = books.get(state.selected_item) {
        b
    } else {
        return;
    };

    let duration_secs = book.duration_seconds.unwrap_or(0) as u64;
    let duration_str = format_duration_detailed(duration_secs);
    let remaining_secs = duration_secs.saturating_sub((duration_secs as f32 * book.progress_percentage() / 100.0) as u64);
    let remaining_str = format_duration_detailed(remaining_secs);

    let chapters_str = if let Some(count) = book.chapter_count {
        format!("{} chapters", count)
    } else {
        "No chapters".to_string()
    };

    let bitrate_str = if let Some(bitrate) = book.bitrate_kbps {
        format!("{} kbps", bitrate)
    } else {
        "Unknown".to_string()
    };

    let narrator_str = book.narrator.clone().unwrap_or_else(|| "Unknown".to_string());
    let series_str = book.series.clone().unwrap_or_else(|| "Standalone".to_string());

    let last_played_str = if let Some(_timestamp) = book.last_played_at {
        "Recently".to_string()  // TODO: Format relative time
    } else {
        "Never".to_string()
    };

    let text = vec![
        Line::from(vec![
            Span::styled("📖 Title: ", theme.accent_style().add_modifier(Modifier::BOLD)),
            Span::styled(&book.title, theme.text_style().add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("✍️  Author: ", theme.accent_style()),
            Span::styled(book.author.clone().unwrap_or_else(|| "Unknown".to_string()), theme.text_style()),
            Span::raw("  |  "),
            Span::styled("🎙️  Narrator: ", theme.accent_style()),
            Span::styled(narrator_str, theme.text_style()),
            Span::raw("  |  "),
            Span::styled("📚 Series: ", theme.accent_style()),
            Span::styled(series_str, theme.text_style()),
        ]),
        Line::from(vec![
            Span::styled("⏱️  Duration: ", theme.accent_style()),
            Span::styled(duration_str, Style::default().fg(Color::Cyan)),
            Span::raw("  |  "),
            Span::styled("⏳ Remaining: ", theme.accent_style()),
            Span::styled(remaining_str, Style::default().fg(Color::Yellow)),
            Span::raw("  |  "),
            Span::styled("📑 ", theme.accent_style()),
            Span::styled(chapters_str, theme.text_style()),
        ]),
        Line::from(vec![
            Span::styled("💾 Size: ", theme.accent_style()),
            Span::styled(format_file_size(book.file_size_bytes.unwrap_or(0)), Style::default().fg(Color::Magenta)),
            Span::raw("  |  "),
            Span::styled("🎵 Bitrate: ", theme.accent_style()),
            Span::styled(bitrate_str, theme.text_style()),
            Span::raw("  |  "),
            Span::styled("🕐 Last Played: ", theme.accent_style()),
            Span::styled(last_played_str, theme.text_secondary_style()),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border_color()))
                .title("📋 Book Details"),
        )
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, area);
}

/// Renders library statistics with calculations
fn render_library_stats(
    frame: &mut Frame,
    area: Rect,
    _state: &AppState,
    theme: &crate::theme::Theme,
    books: &[Book],
) {
    let total_books = books.len();
    let total_duration_secs: u64 = books.iter().map(|b| b.duration_seconds.unwrap_or(0) as u64).sum();
    let total_size_bytes: u64 = books.iter().map(|b| b.file_size_bytes.unwrap_or(0)).sum();

    let completed = books.iter().filter(|b| b.progress_percentage() >= 100.0).count();
    let in_progress = books.iter().filter(|b| b.progress_percentage() > 0.0 && b.progress_percentage() < 100.0).count();
    let unstarted = books.iter().filter(|b| b.progress_percentage() == 0.0).count();

    let avg_completion = if total_books > 0 {
        books.iter().map(|b| b.progress_percentage()).sum::<f32>() / total_books as f32
    } else {
        0.0
    };

    let text = vec![
        Line::from(vec![
            Span::styled("📊 Statistics: ", theme.accent_style().add_modifier(Modifier::BOLD)),
            Span::styled(format!("{} Total Books", total_books), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw("  •  "),
            Span::styled(format!("✓ {} Completed", completed), Style::default().fg(Color::Green)),
            Span::raw("  •  "),
            Span::styled(format!("▶ {} In Progress", in_progress), Style::default().fg(Color::Yellow)),
            Span::raw("  •  "),
            Span::styled(format!("○ {} Not Started", unstarted), Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("📏 Total Duration: ", theme.accent_style()),
            Span::styled(format_duration_detailed(total_duration_secs), Style::default().fg(Color::Cyan)),
            Span::raw("  •  "),
            Span::styled("💾 Total Size: ", theme.accent_style()),
            Span::styled(format_file_size(total_size_bytes), Style::default().fg(Color::Magenta)),
            Span::raw("  •  "),
            Span::styled("📈 Avg Completion: ", theme.accent_style()),
            Span::styled(format!("{:.1}%", avg_completion), Style::default().fg(if avg_completion >= 50.0 { Color::Green } else { Color::Yellow })),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border_color())),
        )
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, area);
}

/// Renders empty library message
fn render_empty_library(frame: &mut Frame, area: Rect, theme: &crate::theme::Theme) {
    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "📚 Your library is empty",
            theme.text_style().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Add audiobooks with:",
            theme.text_secondary_style(),
        )),
        Line::from(Span::styled(
            "  • storystream scan ~/Audiobooks",
            theme.text_style(),
        )),
        Line::from(Span::styled(
            "  • storystream import <file>",
            theme.text_style(),
        )),
    ];

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border_color()))
                .title("📚 Library"),
        )
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

/// Format duration as HH:MM:SS
fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if hours > 0 {
        format!("{}h {:02}m", hours, minutes)
    } else {
        format!("{}m {:02}s", minutes, secs)
    }
}

/// Format duration with full detail
fn format_duration_detailed(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    format!("{}h {}m {}s", hours, minutes, secs)
}

/// Format file size in human-readable format
fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.0} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Create a visual progress bar
fn create_progress_bar(progress: f32, width: usize) -> String {
    let filled = ((progress / 100.0) * width as f32) as usize;
    let empty = width.saturating_sub(filled);

    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}

/// Truncate string to max length with ellipsis
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        format!("{:<width$}", s, width = max_len)
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(0), "0m 00s");
        assert_eq!(format_duration(60), "1m 00s");
        assert_eq!(format_duration(3600), "1h 00m");
        assert_eq!(format_duration(3661), "1h 01m");
    }

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(500), "500 B");
        assert_eq!(format_file_size(1024), "1 KB");
        assert_eq!(format_file_size(1024 * 1024), "1.0 MB");
        assert_eq!(format_file_size(1024 * 1024 * 1024), "1.00 GB");
    }

    #[test]
    fn test_progress_bar() {
        assert_eq!(create_progress_bar(0.0, 10), "[░░░░░░░░░░]");
        assert_eq!(create_progress_bar(50.0, 10), "[█████░░░░░]");
        assert_eq!(create_progress_bar(100.0, 10), "[██████████]");
    }
}