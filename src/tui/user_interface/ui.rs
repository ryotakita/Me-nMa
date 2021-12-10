use crate::tui::user_interface::App;
use std::fs::{read_to_string, read};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, Borders, List, ListItem,
        Paragraph, Tabs, Wrap,
    },
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);
    f.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_first_tab(f, app, chunks[1]),
        _ => {}
    };
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Percentage(80),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(area);
    draw_charts(f, app, chunks[0]);
    draw_text(f, chunks[1]);
}

fn draw_charts<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let constraints = if app.show_chart {
        vec![Constraint::Percentage(50), Constraint::Percentage(50)]
    } else {
        vec![Constraint::Percentage(100)]
    };
    let chunks = Layout::default()
        .constraints(constraints)
        .direction(Direction::Horizontal)
        .split(area);
    {
        let chunks = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .direction(Direction::Horizontal)
            .split(chunks[0]);

        // Draw tasks
        let tasks: Vec<ListItem> = app
            .folders[0]
            .items
            .iter()
            .map(|i| ListItem::new(vec![Spans::from(Span::raw(format!("{}", i)))]))
            .collect();
        let tasks = List::new(tasks)
            .block(Block::default().borders(Borders::ALL).title("Task"))
            //.highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_style(Style::default().fg(Color::Red))
            .highlight_symbol("> ");
        f.render_stateful_widget(tasks, chunks[0], &mut app.folders[0].state);

        //TODO:プレビュー表示
        match app.folders[0].state.selected() {
            Some(x) => {
                let path = app.folders[0].items[x].get_path();
                let contents = match read_to_string(path) {
                    Ok(content) => content,
                    Err(_) => {
                        let s = read(path).unwrap();
                        let (res, _, _) = encoding_rs::SHIFT_JIS.decode(&s);
                        res.into_owned()
                    }
                };
                let text: Vec<Spans> = contents.lines().into_iter().map(|line| Spans::from(line)).collect();
                let block = Block::default().borders(Borders::ALL).title(Span::styled(
                    "Preview",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ));
                let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
                f.render_widget(paragraph, chunks[1]);
            }
            _ => {}
        }
    }
}

fn draw_text<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let text = vec![
        Spans::from(vec![
            Span::from("キー: "),
        ]),
        Spans::from(vec![
            Span::raw("  key\""),
            Span::styled("e", Style::default().add_modifier(Modifier::BOLD).fg(Color::Red)),
            Span::raw("\": "),
            Span::from("システムの終了"),
        ]),
        Spans::from(vec![
            Span::raw("  key\""),
            Span::styled("j", Style::default().add_modifier(Modifier::BOLD).fg(Color::Blue)),
            Span::raw("\": "),
            Span::from("next"),
        ]),
        Spans::from(vec![
            Span::raw("  key\""),
            Span::styled("k", Style::default().add_modifier(Modifier::BOLD).fg(Color::Green)),
            Span::raw("\": "),
            Span::from("pre"),
        ]),
        Spans::from(
            "One more thing is that it should display unicode characters: 10€"
        ),
    ];
    let version = env!("CARGO_PKG_VERSION");
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        format!("Me'nMa {}",version),
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}
