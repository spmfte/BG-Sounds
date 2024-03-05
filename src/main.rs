use std::collections::HashMap;
use std::io;
use std::process::Command;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Gauge, ListItem, Paragraph};
use tui::widgets::canvas::Canvas;
use tui::Terminal;
use tui::widgets::ListState;
use termion::input::TermRead;
use termion::event::Key;
use rand::Rng;

lazy_static::lazy_static! {
    static ref SOUND_MAPPINGS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Balanced Noise", "{length = 1720, bytes = 0x62706c69 73743030 d4010203 04050607 ... 00000000 00000596}");
        m.insert("Bright Noise", "{length = 1726, bytes = 0x62706c69 73743030 d4010203 04050607 ... 00000000 0000059a}");
        m.insert("Dark Noise", "{length = 1726, bytes = 0x62706c69 73743030 d4010203 04050607 ... 00000000 0000059a}");
        m.insert("Ocean", "{length = 1721, bytes = 0x62706c69 73743030 d4010203 04050607 ... 00000000 00000595}");
        m.insert("Rain", "{length = 1720, bytes = 0x62706c69 73743030 d4010203 04050607 ... 00000000 00000594}");
        m.insert("Stream", "{length = 1722, bytes = 0x62706c69 73743030 d4010203 04050607 ... 00000000 00000596}");
        m
    };
}

struct App {
    size: Rect,
    dialogue: bool,
    selected: usize,
    items: Vec<String>,
    volume: u16,
    state: ListState,
    sound_playing: bool,
}

impl App {
    fn new() -> App {
        App {
            sound_playing: false,
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
    let mut stdin = io::stdin().keys();

    terminal.clear()?;

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Percentage(30),
                    Constraint::Percentage(20),
                    Constraint::Percentage(20),
                    Constraint::Percentage(30),
                ].as_ref())
                .split(f.size());

            let items: Vec<_> = app.items.iter().map(|i| ListItem::new(i.as_str())).collect();
            app.state.select(Some(app.selected));
            let list = tui::widgets::List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Select a sound"))
                .highlight_style(Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD))
                .highlight_symbol(">>");

            f.render_stateful_widget(list, chunks[0], &mut app.state);

            let message = if app.sound_playing {
                "Sound is playing..."
            } else {
                "Sound is not playing..."
            };

            let paragraph = Paragraph::new(message)
                .block(Block::default().borders(Borders::ALL).title("Status"));
            f.render_widget(paragraph, chunks[1]);

            let gauge = Gauge::default()
                .block(Block::default().title("Volume"))
                .gauge_style(Style::default().fg(Color::Yellow))
                .percent(app.volume);
            f.render_widget(gauge, chunks[2]);

            let mut rng = rand::thread_rng();
            let random_values: Vec<(f64, f64)> = (0..10).map(|i| (i as f64, rng.gen_range(0.0..100.0))).collect();

            let canvas = Canvas::default()
                .block(Block::default().title("Visualizer"))
                .paint(|ctx| {
                    let points = tui::widgets::canvas::Points { coords: &random_values, color: Color::White };
                    ctx.draw(&points);
                })
                .x_bounds([0.0, 10.0])
                .y_bounds([0.0, 100.0]);

            f.render_widget(canvas, chunks[3]);
        })?;

        let input = stdin.next();

        if let Some(Ok(key)) = input {
            match key {
                Key::Char('j') => {
                    if app.selected < app.items.len() - 1 {
                        app.selected += 1;
                    }
                }
                Key::Char('k') => {
                    if app.selected > 0 {
                        app.selected -= 1;
                    }
                }
                Key::Char('u') => {
                    if app.volume < 100 {
                        app.volume += 1;
                    }
                }
                Key::Char('d') => {
                    if app.volume > 0 {
                        app.volume -= 1;
                    }
                }
                Key::Char('\n') => {
                    app.sound_playing = !app.sound_playing;

                    let sound_option = &app.items[app.selected];
                    if let Some(sound_data) = SOUND_MAPPINGS.get(sound_option.as_str()) {
                        // Write the sound data
                        let status = Command::new("defaults")
                            .arg("write")
                            .arg("com.apple.ComfortSounds")
                            .arg("ComfortSoundsSelectedSound")
                            .arg("-data")
                            .arg(sound_data)
                            .status()?;

                        if !status.success() {
                            eprintln!("Failed to set the sound data for {}", sound_option);
                        }
                    }

                    let enable = app.sound_playing;
                    let status = Command::new("defaults")
                        .arg("write")
                        .arg("com.apple.ComfortSounds")
                        .arg("comfortSoundsEnabled")
                        .arg("-bool")
                        .arg(enable.to_string())
                        .status()?;

                    if !status.success() {
                        eprintln!("Failed to write to com.apple.ComfortSounds");
                    }

                    let status = Command::new("launchctl")
                        .arg("kill")
                        .arg("SIGHUP")
                        .arg(format!("gui/{}/com.apple.accessibility.heard", users::get_current_uid()))
                        .status()?;

                    if !status.success() {
                        eprintln!("Failed to send SIGHUP to com.apple.accessibility.heard");
                    }
                }
                Key::Ctrl('c') => {
                    terminal.clear()?;
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

