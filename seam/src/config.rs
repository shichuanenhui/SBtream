use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::util::bin_dir;

// TODO 接受 headermap 传入, 由具体实现选择是否使用

#[derive(Deserialize, Debug)]
pub struct Config {
    pub video: VideoConfig,
    pub danmu: DanmuConfig,
    pub cookie: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Debug)]
pub struct VideoConfig {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct DanmuConfig {
    pub name: String,
}

/// 配置文件
pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let config = std::fs::read_to_string(format!("{}config.toml", bin_dir(),)).unwrap();
    basic_toml::from_str::<Config>(&config).unwrap()
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        // 初始化 CONFIG
        let _ = CONFIG.video.name.clone();
        println!("{:#?}", CONFIG);
    }
}
