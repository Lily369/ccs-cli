use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal,
};
use std::io::{self, Write};

use crate::db::Provider;

const PAGE_SIZE: usize = 9;

fn print_page(providers: &[Provider], page: usize) {
    let total_pages = (providers.len() + PAGE_SIZE - 1) / PAGE_SIZE;
    let start = page * PAGE_SIZE;
    let end = (start + PAGE_SIZE).min(providers.len());

    let mut stdout = io::stdout();

    // 清屏并回到左上角
    execute!(stdout, terminal::Clear(terminal::ClearType::All), crossterm::cursor::MoveTo(0, 0)).ok();

    let lines = {
        let mut v: Vec<String> = Vec::new();
        v.push("  ccs ── Claude Code Switcher".to_string());
        v.push("".to_string());
        for (i, provider) in providers[start..end].iter().enumerate() {
            v.push(format!("  {}. {}", i + 1, provider.name));
        }
        v.push("".to_string());
        if total_pages > 1 {
            v.push(format!(
                "  第 {}/{} 页  u/i 翻页  1-{} 选择  q 退出",
                page + 1,
                total_pages,
                end - start
            ));
        } else {
            v.push(format!("  1-{} 选择  q 退出", end - start));
        }
        v
    };

    for line in &lines {
        let _ = write!(stdout, "{}\r\n", line);
    }
    let _ = stdout.flush();
}

pub fn select(providers: &[Provider]) -> Option<&Provider> {
    let total_pages = (providers.len() + PAGE_SIZE - 1) / PAGE_SIZE;
    let mut page: usize = 0;

    terminal::enable_raw_mode().expect("无法启用 raw mode");
    print_page(providers, page);

    let result = loop {
        if let Ok(Event::Key(KeyEvent { code, .. })) = event::read() {
            match code {
                KeyCode::Char('q') | KeyCode::Esc => break None,
                KeyCode::Char('u') => {
                    if page > 0 {
                        page -= 1;
                        print_page(providers, page);
                    }
                }
                KeyCode::Char('i') => {
                    if page + 1 < total_pages {
                        page += 1;
                        print_page(providers, page);
                    }
                }
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    let idx = c.to_digit(10).unwrap() as usize;
                    if idx >= 1 && idx <= PAGE_SIZE {
                        let real_idx = page * PAGE_SIZE + idx;
                        if real_idx <= providers.len() {
                            break Some(&providers[real_idx - 1]);
                        }
                    }
                }
                _ => {}
            }
        }
    };

    let _ = terminal::disable_raw_mode();
    let mut stdout = io::stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All), crossterm::cursor::MoveTo(0, 0)).ok();
    result
}
