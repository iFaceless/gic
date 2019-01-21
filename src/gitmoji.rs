use serde_derive::Deserialize;
use serde_json;

static GITMOJI_CONFIG: &str = r#"
[
  {
    "name": "init",
    "emoji": "🎉",
    "code": ":tada:",
    "description": "✨ Initial commit"
  }
]
"#;

#[derive(Debug, Deserialize)]
pub struct Gitmoji {
    pub name: String,
    pub emoji: String,
    pub code: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct GitmojiConfig {
    pub gitmojis: Vec<Gitmoji>,
}

impl GitmojiConfig {
    pub fn new() -> Self {
        let gitmojis: Vec<Gitmoji> = serde_json::from_str(GITMOJI_CONFIG).unwrap_or_default();
        GitmojiConfig { gitmojis }
    }
}
