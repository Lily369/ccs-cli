use std::fs;
use std::path::PathBuf;
use std::process;

use crate::db::Provider;

// cc-switch 自己的内部元数据字段，claude 看不懂，写入前剔掉。
const SANITIZE_KEYS: &[&str] = &[
    "api_format",
    "apiFormat",
    "openrouter_compat_mode",
    "openrouterCompatMode",
];

fn temp_config_path(provider_id: &str) -> PathBuf {
    #[cfg(windows)]
    {
        PathBuf::from(format!(
            "{}\\ccs-{}.json",
            std::env::temp_dir().display(),
            provider_id
        ))
    }
    #[cfg(not(windows))]
    {
        PathBuf::from(format!("/tmp/ccs-{}.json", provider_id))
    }
}

pub fn exec_claude(
    provider: &Provider,
    skip_permissions: bool,
    dry_run: bool,
    resume: Option<Option<String>>,
) {
    // 拷贝 cc-switch 的整份 settings_config，仅剔掉 cc-switch 自身的元数据字段。
    // env 块保留在 JSON 里，由 claude --settings 自行解析。
    let mut config = provider.settings_config.clone();
    if let Some(obj) = config.as_object_mut() {
        for key in SANITIZE_KEYS {
            obj.remove(*key);
        }
    }

    let temp_path = temp_config_path(&provider.id);
    let json = serde_json::to_string_pretty(&config).expect("JSON 序列化失败");
    fs::write(&temp_path, &json).unwrap_or_else(|e| {
        eprintln!("写入临时配置文件失败: {e}");
        process::exit(1);
    });

    let mut cmd = process::Command::new("claude");
    cmd.args(["--settings", temp_path.to_str().unwrap_or("")]);
    if let Some(session) = &resume {
        cmd.arg("--resume");
        if let Some(id) = session {
            cmd.arg(id);
        }
    }
    if skip_permissions {
        cmd.arg("--dangerously-skip-permissions");
    }

    if dry_run {
        println!("=== ccs --print (不会启动 claude) ===");
        println!("provider: {} ({})", provider.name, provider.id);
        println!("temp settings: {}", temp_path.display());
        println!("\nargv:");
        print!("  claude --settings {}", temp_path.display());
        if let Some(session) = &resume {
            print!(" --resume");
            if let Some(id) = session {
                print!(" {id}");
            }
        }
        if skip_permissions {
            print!(" --dangerously-skip-permissions");
        }
        println!();
        println!("\nsettings 文件内容（已写入）:");
        println!("{}", json);
        process::exit(0);
    }

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        let err = cmd.exec();
        eprintln!("启动 claude 失败: {err}");
        process::exit(1);
    }

    #[cfg(windows)]
    {
        let status = cmd.status().unwrap_or_else(|e| {
            eprintln!("启动 claude 失败: {e}");
            process::exit(1);
        });
        process::exit(status.code().unwrap_or(1));
    }
}
