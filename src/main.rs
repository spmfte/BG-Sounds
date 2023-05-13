use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use crossterm::event::{self, Event as CEvent, KeyCode};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

enum AppEvent<I> {
    Input(I),
}

struct App {
    size: Rect,
    dialogue: bool,
    sounds: Vec<&'static str>,
    selected_sound: usize,
    volume: u8,
}

impl Default for App {
    fn default() -> App {
        App {
            size: Rect::default(),
            dialogue: false,
            sounds: vec![
                "Balanced Noise",
                "Bright Noise",
                "Dark Noise",
                "Ocean",
                "Rain",
                "Stream",
            ],
            selected_sound: 0,
            volume: 50,
        }
    }
}

fn main() -> Result<(), io::Error> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let tick_rate = Duration::from_millis(200);
        loop {
            if event::poll(tick_rate - Duration::from_millis(1)).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    tx.send(AppEvent::Input(key)).unwrap();
                }
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::default();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let block = Block::default().title("Background Sounds").borders(Borders::ALL);
            f.render_widget(block, chunks[0]);

            let items: Vec<ListItem> = app.sounds
                .iter()
                .map(|i| {
                    ListItem::new(vec![Spans::from(Span::styled(
                        *i,
                        Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                    ))])
                })
                .collect();

            let sounds_list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Select a sound"))
                .highlight_style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">>");

            f.render_stateful_widget(sounds_list, chunks[1], &mut app.selected_sound);

            let quit_block = Block::default().title("Press 'q' to quit").borders(Borders::ALL);
            f.render_widget(quit_block, chunks[2]);

            // Let's also display the volume level
            let volume_str = format!(
                "Volume: [{}{}] {}%",
                "#".repeat(app.volume as usize / 10),
                " ".repeat(10 - app.volume as usize / 10),
                app.volume
            );
            let volume_paragraph = Paragraph::new(volume_str)
                .style(Style::default().fg(Color::White))
                .block(Block::default().borders(Borders::ALL).title("Volume"));
            f.render_widget(volume_paragraph, chunks[2]);
        })?;

        match rx.recv()? {
            AppEvent::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    break;
                }
                KeyCode::Down => {
                    app.selected_sound = if app.selected_sound >= app.sounds.len() - 1 {
                        app.sounds.len() - 1
                    } else {
                        app.selected_sound + 1
                    };
                }
                KeyCode::Up => {
                    app.selected_sound = if app.selected_sound > 0 {
                        app.selected_sound - 1
                    } else {
                        0
                    };
                }
                KeyCode::Enter => {
                    // Call the function to play sound here
                    play_sound(&app.sounds[app.selected_sound]);
                }
                KeyCode::Char('-') => {
                    app.volume = app.volume.saturating_sub(10);
                    // Set the volume here
                    set_volume(app.volume);
                }
                KeyCode::Char('+') => {
                    app.volume = (app.volume + 10).min(100);
                    // Set the volume here
                    set_volume(app.volume);
                }
                _ => {}
            },
        }
    }

    Ok(())
}

fn play_sound(sound: &str) {
    let _ = Command::new("sh")
        .arg("-c")
        .arg(format!("defaults write com.apple.ComfortSounds ComfortSoundsSelectedSound -string {}", sound))
        .output()
        .expect("Failed to set background sound");
        
    let _ = Command::new("sh")
        .arg("-c")
        .arg("launchctl kill SIGHUP gui/$(id -u)/com.apple.accessibility.heard")
        .output()
        .expect("Failed to send signal");
}

fn set_volume(volume: i32) {
    let volume_f64 = volume as f64 / 100.0;
    
    let _ = Command::new("sh")
        .arg("-c")
        .arg(format!("defaults write com.apple.ComfortSounds relativeVolume -float {}", volume_f64))
        .output()
        .expect("Failed to set volume");
        
    let _ = Command::new("sh")
        .arg("-c")
        .arg("launchctl kill SIGHUP gui/$(id -u)/com.apple.accessibility.heard")
        .output()
        .expect("Failed to send signal");
}

