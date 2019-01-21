use serde_derive::Deserialize;
use serde_json;

static GITMOJI_CONFIG: &str = r#"
[
  {
    "name": "feat",
    "emoji": "✨",
    "code": ":sparkles:",
    "description": "✨ Introducing new features."
  },
  {
    "name": "fix",
    "emoji": "🐛",
    "code": ":bug:",
    "description": "🐛 Fixing a bug."
  },
  {
    "name": "docs",
    "emoji": "📝",
    "code": ":memo:",
    "description": "📝 Writing docs."
  },
  {
    "name": "style",
    "emoji": "💄",
    "code": ":lipstick:",
    "description": "💄 Updating the UI and style files."
  },
  {
    "name": "refactor",
    "emoji": "🔨",
    "code": ":wrench:",
    "description": "🔨 Refactoring code."
  },
  {
    "name": "perf",
    "emoji": "⚡️",
    "code": ":zap:",
    "description": "⚡ Improving performance."
  },
  {
    "name": "test",
    "emoji": "✅",
    "code": ":white_check_mark:",
    "description": "✅ Adding tests."
  },
  {
    "name": "clean",
    "emoji": "🔥",
    "code": ":fire:",
    "description": "🔥 Removing code or files."
  },
  {
    "name": "hotfix",
    "emoji": "🚑",
    "code": ":ambulance:",
    "description": "🚑 Critical hotfix."
  },
  {
    "name": "chore",
    "emoji": "🚀",
    "code": ":rocket:",
    "description": "🚀 Update build scripts, no production code change."
  },
  {
    "name": "init",
    "emoji": "🎉",
    "code": ":tada:",
    "description": "🎉 Initial commit"
  },
  {
    "name": "security",
    "emoji": "🔒",
    "code": ":lock:",
    "description": "🔒 Fixing security issues."
  },
  {
    "name": "tag",
    "emoji": "🔖",
    "code": ":bookmark:",
    "description": "🔖 Releasing / Version tags"
  },
  {
    "name": "lintfix",
    "emoji": "🚨",
    "code": ":rotating_light:",
    "description": "🚨 Removing linter warnings."
  },
  {
    "name": "wip",
    "emoji": "🚧",
    "code": ":construction:",
    "description": "🚧 Work in progress."
  },
  {
    "name": "conf",
    "emoji": "🔧",
    "code": ":wrench:",
    "description": "🔧 Changing configuration files."
  },
  {
    "name": "rename",
    "emoji": "🚚",
    "code": ":truck:",
    "description": "🚚 Moving or renaming files."
  },
  {
    "name": "break",
    "emoji": "💥",
    "code": ":boom:",
    "description": "💥 Introducing breaking changes."
  },
  {
    "name": "shit",
    "emoji": "💩",
    "code": ":shit:",
    "description": "💩 Dealing with shits."
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
