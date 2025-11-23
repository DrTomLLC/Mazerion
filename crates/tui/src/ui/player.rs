// crates/tui/src/ui/player.rs
//! Enhanced player view with detailed playback information and beautiful visualizations

use crate::state::AppState;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Sparkline},
    Frame,
};
use std::time::Duration;

/// Renders the enhanced player view with comprehensive information
pub fn render(frame: &mut Frame, area: Rect, state: &AppState, theme: &crate::theme::Theme) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),  // Now playing info
            Constraint::Length(5),  // Progress bar with time
            Constraint::Length(4),  // Playback stats (speed, volume, chapter)
            Constraint::Length(8),  // Controls help
            Constraint::Min(0),     // Chapter/bookmark info
        ])
        .split(area);

    render_now_playing_enhanced(frame, chunks[0], state, theme);
    render_progress_enhanced(frame, chunks[1], state, theme);
    render_playback_stats(frame, chunks[2], state, theme);
    render_controls_help(frame, chunks[3], state, theme);
    render_chapter_bookmark_info(frame, chunks[4], state, theme);
}

/// Renders detailed now playing information
fn render_now_playing_enhanced(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
    theme: &crate::theme::Theme,
) {
    let (title, author, file_info) = if let Some(ref file) = state.playback.current_file {
        // TODO: Get actual book info
        (file.clone(), "Unknown Author".to_string(), "MP3 • 128 kbps • 44.1 kHz".to_string())
    } else {
        (
            "No audiobook loaded".to_string(),
            String::new(),
            String::new(),
        )
    };

    let status_icon = if state.playback.is_playing {
        ("▶", Color::Green)
    } else {
        ("⏸", Color::Yellow)
    };

    let text = vec![
        Line::from(vec![
            Span::styled(
                format!("{} ", status_icon.0),
                Style::default().fg(status_icon.1).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "NOW PLAYING",
                theme.accent_style().add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            truncate(&title, 80),
            theme.text_style().add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!("by {}", author),
            theme.text_secondary_style(),
        )),
        Line::from(""),
        Line::from(Span::styled(file_info, Style::default().fg(Color::DarkGray))),
    ];

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border_color())),
        )
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

/// Renders enhanced progress bar with time calculations
fn render_progress_enhanced(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
    theme: &crate::theme::Theme,
) {
    let progress_percent = (state.playback.progress() * 100.0) as u16;

    let position = state.playback.position;
    let duration = state.playback.duration;
    let remaining = duration.saturating_sub(position);

    // Calculate time to finish at current speed
    let speed = state.playback.speed;
    let remaining_at_speed = if speed > 0.0 {
        Duration::from_secs_f32(remaining.as_secs_f32() / speed)
    } else {
        remaining
    };

    let current_time = format_time(position);
    let total_time = format_time(duration);
    let remaining_time = format_time(remaining);
    let eta_time = format_time(remaining_at_speed);

    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border_color()))
                .title("⏳ Playback Progress"),
        )
        .gauge_style(
            Style::default()
                .fg(Color::Cyan)
                .bg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .label(format!("{:.1}%", progress_percent as f32 / 10.0))
        .percent(progress_percent);

    frame.render_widget(gauge, area);

    // Render time information below the gauge
    let time_area = Rect {
        y: area.y + 2,
        height: 2,
        ..area
    };

    let time_text = vec![
        Line::from(vec![
            Span::styled(format!("⏱️  Position: {}", current_time), Style::default().fg(Color::Cyan)),
            Span::raw("  •  "),
            Span::styled(format!("Duration: {}", total_time), Style::default().fg(Color::Cyan)),
            Span::raw("  •  "),
            Span::styled(format!("Remaining: {}", remaining_time), Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::styled(format!("🎯 ETA (at {:.2}x): {}", speed, eta_time), Style::default().fg(Color::Green)),
            Span::raw("  •  "),
            Span::styled(
                format!("📊 Progress: {}%", progress_percent),
                if progress_percent >= 90 {
                    Style::default().fg(Color::Green)
                } else if progress_percent >= 50 {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default().fg(Color::White)
                },
            ),
        ]),
    ];

    let time_paragraph = Paragraph::new(time_text).alignment(Alignment::Center);

    frame.render_widget(time_paragraph, time_area);
}

/// Renders playback statistics
fn render_playback_stats(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
    theme: &crate::theme::Theme,
) {
    let speed = state.playback.speed;
    let volume_percent = (state.playback.volume * 100.0) as u8;
    let chapter_info = state.playback.current_chapter
        .as_ref()
        .map(|ch| format!("Chapter {} of {}: {}", ch.0, ch.1, ch.2))
        .unwrap_or_else(|| "No chapters".to_string());

    // Calculate time saved/added by speed change
    let time_diff = if speed != 1.0 {
        let original_duration = state.playback.duration.as_secs_f32();
        let adjusted_duration = original_duration / speed;
        let diff_secs = original_duration - adjusted_duration;
        if diff_secs > 0.0 {
            format!("⚡ Saving {} ", format_time(Duration::from_secs_f32(diff_secs.abs())))
        } else {
            format!("🐌 Adding {} ", format_time(Duration::from_secs_f32(diff_secs.abs())))
        }
    } else {
        String::new()
    };

    let text = vec![
        Line::from(vec![
            Span::styled("🎚️  Playback: ", theme.accent_style().add_modifier(Modifier::BOLD)),
            Span::styled(
                format!("Speed: {:.2}x", speed),
                Style::default().fg(if speed > 1.0 { Color::Green } else if speed < 1.0 { Color::Yellow } else { Color::White }),
            ),
            Span::raw("  •  "),
            Span::styled(
                format!("Volume: {}%", volume_percent),
                Style::default().fg(if volume_percent >= 70 { Color::Green } else if volume_percent >= 40 { Color::Yellow } else { Color::Red }),
            ),
            Span::raw("  "),
            Span::styled(create_volume_bar(volume_percent, 10), Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::styled("📑 ", theme.accent_style()),
            Span::styled(truncate(&chapter_info, 60), theme.text_style()),
            Span::raw("  "),
            Span::styled(time_diff, Style::default().fg(Color::Green)),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border_color()))
                .title("📊 Stats"),
        )
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, area);
}

/// Renders keyboard controls help
fn render_controls_help(
    frame: &mut Frame,
    area: Rect,
    _state: &AppState,
    theme: &crate::theme::Theme,
) {
    let text = vec![
        Line::from(vec![
            Span::styled("⌨️  Controls: ", theme.accent_style().add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Space", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(": Play/Pause  •  "),
            Span::styled("←/→", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(": Seek ±10s  •  "),
            Span::styled("[/]", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(": Speed ±0.1x  •  "),
            Span::styled("+/-", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(": Volume ±5%"),
        ]),
        Line::from(vec![
            Span::styled("n/p", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(": Next/Prev Chapter  •  "),
            Span::styled("b", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(": Bookmark  •  "),
            Span::styled("r", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(": Restart  •  "),
            Span::styled("s", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(": Sleep Timer"),
        ]),
        Line::from(vec![
            Span::styled("Shift+←/→", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(": Seek ±1m  •  "),
            Span::styled("Ctrl+←/→", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(": Seek ±5m  •  "),
            Span::styled("1-9", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(": Jump to position"),
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

/// Renders chapter and bookmark information
fn render_chapter_bookmark_info(
    frame: &mut Frame,
    area: Rect,
    _state: &AppState,
    theme: &crate::theme::Theme,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Chapters
    let chapter_text = vec![
        Line::from(Span::styled(
            "📑 Chapters",
            theme.accent_style().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("  1. Introduction (0:00)"),
        Line::from("  2. Chapter One (5:23)"),
        Line::from("  3. Chapter Two (15:45) ◀"),
        Line::from("  4. Chapter Three (28:12)"),
        Line::from("  5. Chapter Four (42:03)"),
    ];

    let chapter_paragraph = Paragraph::new(chapter_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border_color())),
        )
        .alignment(Alignment::Left);

    frame.render_widget(chapter_paragraph, chunks[0]);

    // Bookmarks
    let bookmark_text = vec![
        Line::from(Span::styled(
            "🔖 Bookmarks",
            theme.accent_style().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  ★ ", Style::default().fg(Color::Yellow)),
            Span::raw("Important quote (12:34)"),
        ]),
        Line::from(vec![
            Span::styled("  ★ ", Style::default().fg(Color::Yellow)),
            Span::raw("Resume here (18:52)"),
        ]),
        Line::from(vec![
            Span::styled("  ★ ", Style::default().fg(Color::Yellow)),
            Span::raw("Great scene (35:17)"),
        ]),
        Line::from(""),
        Line::from(Span::styled("Press 'b' to add bookmark", theme.text_secondary_style())),
    ];

    let bookmark_paragraph = Paragraph::new(bookmark_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border_color())),
        )
        .alignment(Alignment::Left);

    frame.render_widget(bookmark_paragraph, chunks[1]);
}

/// Format duration as HH:MM:SS
fn format_time(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{:02}:{:02}", minutes, seconds)
    }
}

/// Create a visual volume bar
fn create_volume_bar(volume: u8, width: usize) -> String {
    let filled = ((volume as f32 / 100.0) * width as f32) as usize;
    let empty = width.saturating_sub(filled);

    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}

/// Truncate string with ellipsis
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_time() {
        assert_eq!(format_time(Duration::from_secs(0)), "00:00");
        assert_eq!(format_time(Duration::from_secs(65)), "01:05");
        assert_eq!(format_time(Duration::from_secs(3661)), "01:01:01");
    }

    #[test]
    fn test_volume_bar() {
        assert_eq!(create_volume_bar(0, 10), "[░░░░░░░░░░]");
        assert_eq!(create_volume_bar(50, 10), "[█████░░░░░]");
        assert_eq!(create_volume_bar(100, 10), "[██████████]");
    }
}