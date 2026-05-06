use rusqlite::Connection;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Provider {
    pub id: String,
    pub name: String,
    pub settings_config: serde_json::Value,
}

pub fn load_claude_providers(db_path: &Path) -> Result<Vec<Provider>, String> {
    if !db_path.exists() {
        return Err(format!(
            "cc-switch 数据库不存在: {}\n请先安装并配置 cc-switch",
            db_path.display()
        ));
    }

    let conn = Connection::open(db_path).map_err(|e| format!("打开数据库失败: {e}"))?;

    let mut stmt = conn
        .prepare(
            "SELECT id, name, settings_config \
             FROM providers \
             WHERE app_type = 'claude' \
             ORDER BY COALESCE(sort_index, 999999), created_at ASC, id ASC",
        )
        .map_err(|e| format!("准备查询失败: {e}"))?;

    let providers = stmt
        .query_map([], |row| {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let config_str: String = row.get(2)?;
            let settings_config: serde_json::Value =
                serde_json::from_str(&config_str).unwrap_or(serde_json::Value::Null);
            Ok(Provider {
                id,
                name,
                settings_config,
            })
        })
        .map_err(|e| format!("查询供应商失败: {e}"))?
        .filter_map(|r| r.ok())
        .collect();

    Ok(providers)
}
