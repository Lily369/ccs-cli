mod db;
mod launcher;
mod tui;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("--version" | "-v") => {
            println!("ccs {}", env!("CARGO_PKG_VERSION"));
            process::exit(0);
        }
        Some("--help" | "-h") => {
            println!("ccs - Claude Code Switcher\n");
            println!("用法: ccs [选项]\n");
            println!("选项:");
            println!("  --version, -v  显示版本");
            println!("  -dsp           跳过权限确认 (dangerously-skip-permissions)");
            println!("  --print        只打印将注入的 env / argv，不启动 claude（调试用）");
            println!("  --help, -h     显示帮助");
            process::exit(0);
        }
        _ => {}
    }

    let home = dirs::home_dir().unwrap_or_else(|| {
        eprintln!("无法获取用户主目录");
        process::exit(1);
    });

    let db_path = home.join(".cc-switch").join("cc-switch.db");

    let providers = db::load_claude_providers(&db_path).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });

    if providers.is_empty() {
        eprintln!("未找到 Claude 供应商，请先在 cc-switch 中添加");
        process::exit(1);
    }

    let skip_permissions = args.contains(&"-dsp".to_string());
    let dry_run = args.contains(&"--print".to_string());

    match tui::select(&providers) {
        Some(provider) => launcher::exec_claude(provider, skip_permissions, dry_run),
        None => process::exit(0),
    }
}
