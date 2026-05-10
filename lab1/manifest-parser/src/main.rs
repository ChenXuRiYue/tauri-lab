mod plugin_json;

use plugin_json::scan_plugins;
use std::{path::Path};

fn main() {
    // 读取文件夹
    let json_path = Path::new("json_plugin");
    for (plugin_dir, result) in scan_plugins(json_path) {
        let display_name = plugin_dir
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");

        match result {
            Ok(manifest) => {
                println!(
                    "OK {display_name}: v{}, {} commands",
                    manifest.version,
                    manifest.commands.len(),
                )
            }
            Err(error) => {
                println!("Err {display_name}: {error}");
            }
        }
    }

}