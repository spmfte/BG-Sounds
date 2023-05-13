use std::io;
use std::process::Command;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, ListItem};
use tui::Terminal;
use tui::widgets::List;
use tui::widgets::ListState;


struct App {
    size: Rect,
    dialogue: bool,
    selected: usize,
    items: Vec<String>,
    volume: u8,
    state: ListState,
}

impl App {
    fn new() -> App {
        App {
            size: Rect::default(),
            dialogue: false,
            selected: 0,
            items: vec![
                "Balanced Noise".into(),
                "Bright Noise".into(),
                "Dark Noise".into(),
                "Ocean".into(),
                "Rain".into(),
                "Stream".into(),
            ],
            volume: 0,
            state: ListState::default(),
        }
    }
}

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    terminal.clear()?;

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());

            let items: Vec<_> = app.items.iter().map(|i| ListItem::new(i.as_str())).collect();
            app.state.select(Some(app.selected));
            let list = tui::widgets::List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Select a sound"))
                .highlight_style(Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD))
                .highlight_symbol(">>");

            f.render_stateful_widget(list, chunks[0], &mut app.state);
        })?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim() == "quit" {
            break;
        }

        match input.trim().parse::<usize>() {
            Ok(i) if i > 0 && i <= app.items.len() => {
                app.selected = i - 1;

                // Turn on/off the background sounds
                let output = if app.selected % 2 == 0 {
                    Command::new("defaults")
                        .arg("write")
                        .arg("com.apple.ComfortSounds")
                        .arg("comfortSoundsEnabled")
                        .arg("-bool")
                        .arg("true")
                        .output()
                        .expect("Failed to turn on the sound")
                } else {
                    Command::new("defaults")
                        .arg("write")
                        .arg("com.apple.ComfortSounds")
                        .arg("comfortSoundsEnabled")
                        .arg("-bool")
                        .arg("false")
                        .output()
                        .expect("Failed to turn off the sound")
                };

                // Signal the process to re-read the configuration
                Command::new("launchctl")
                    .arg("kill")
                    .arg("SIGHUP")
                    .arg("gui/501/com.apple.accessibility.heard")
                    .output()
                    .expect("Failed to signal the process");

                if !output.status.success() {
                    eprintln!("Command executed with error: {}", output.status);
                }
            }
            _ => eprintln!("Invalid input"),
        }
    }

    Ok(())
}

