use serde::Deserialize;
use std::fs;

// 
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
// snake 驼峰方式等小映射
#[serde(rename_all = "camelCase")] 
pub struct PluginJsonManifest {
    pub name: String,
    pub plugin_type: PluginType,
    pub version: String,
    pub github_url: String,
    pub plugin_version: String,

    #[serde(default)]
    pub commands: Vec<CommandDecl>
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PluginType {
    Internal,
    Wasm
}


// comond 
#[derive(Debug, Deserialize, PartialEq, Eq)]
// 避免 manifest 静默接受拼写错误或无效字段
#[serde(deny_unknown_fields)]
pub struct CommandDecl {
    pub id: String,
    pub description: String,
}


// 解析 mainifest
// Path 代表一个文件系统路径的不可变借用。它不拥有路径数据的所有权，只是提供一个查看路径的途径。
pub fn parse_manifest(path: &Path) -> Result<PluginJsonManifest, Box<dyn std::error::Error>> {
    let raw = fs::read_to_string(path)?;
    let manifest = serde_json::from_str(&raw)?;
    Ok(manifest)
}

// 扫描文件目录下的插件
pub fn scan_plugins(
    dir: &Path,
    // 下边这个返回值是个啥玩意
) ->Vec<(PathBuf, Result<PluginJsonManifest, String>)> {
    let mut results = Vec::new();

    // 读取目录文件

    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(err) => {
            results.push((dir.to_path_buf(), Err(err.to_string())));
            return results;
        }
    };

    for entry in entries {
        match entry {
            Ok(entry) => {
                let plugin_dir = entry.path();
                if !plugin_dir.is_dir() {
                    continue;
                }

                let manifest_path = plugin_dir.join("manifest.json");
                let parsed = parse_manifest(&manifest_path)
                        .map_err(|err| err.to_string());
                results.push((plugin_dir, parsed));
            }
            Err(err) => results.push((dir.to_path_buf(), Err(err.to_string()))),
        }
    }

    results
}

