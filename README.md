```
      ___                       ___
     /\__\                     /\__\
    /:/ _/_       ___         /:/  /
   /:/ /\  \     /\__\       /:/  /
  /:/ /::\  \   /:/__/      /:/  /  ___
 /:/__\/\:\__\ /::\  \     /:/__/  /\__\
 \:\  \ /:/  / \/\:\  \__  \:\  \ /:/  /
  \:\  /:/  /   ~~\:\/\__\  \:\  /:/  /
   \:\/:/  /       \::/  /   \:\/:/  /
    \::/  /        /:/  /     \::/  /
     \/__/         \/__/       \/__/

```

# What's gic

[gic](https://github.com/iFaceless/gic) is a very simple but powerful command line tool, which aims at generating uniform and elegant commit message for you.

And more importantly, **it's implemented in Rust~** :)

Inspired by the following projects:
1. guitar
1. [Semantic Commit Messages with Emojis](https://medium.com/walmartlabs/semantic-commit-messages-with-emojis-dba2541cea9a)
1. [gitmoji](https://gitmoji.carloscuesta.me/)
1. [GitCommitEmoji](https://gist.github.com/parmentf/035de27d6ed1dce0b36a)

# Install

```
wget -q https://raw.githubusercontent.com/iFaceless/gic/master/install.sh | sh install.sh
```

# Usage

![gic](screenshots/gic.gif)

Run `gic --help`, you'll get all the available choices:

```
gic <version number>
Generate elegant and uniform commit messages~

USAGE:
    gic [FLAGS] [OPTIONS] <message>

FLAGS:
    -a, --amend       amend commit message
        --break       💥 Introducing breaking changes.
        --chore       🚀 Update build scripts, no production code change.
        --clean       🔥 Removing code or files.
        --conf        🔧 Changing configuration files.
        --docs        📝 Writing docs.
        --feat        ✨ Introducing new features.
        --fix         🐛 Fixing a bug.
    -h, --help        Prints help information
        --hotfix      🚑 Critical hotfix.
        --init        🎉 Initial commit
        --lintfix     🚨 Removing linter warnings.
        --perf        ⚡ Improving performance.
        --refactor    🔨 Refactoring code.
        --rename      🚚 Moving or renaming files.
        --security    🔒 Fixing security issues.
        --shit        💩 Dealing with shits.
        --style       💄 Updating the UI and style files.
        --tag         🔖 Releasing / Version tags
        --test        ✅ Adding tests.
    -V, --version     Prints version information
        --wip         🚧 Work in progress.

OPTIONS:
    -i, --issue <issue>...    Related issue ID
    -j, --jira <jira>...      Related JIRA task ID
    -s, --scope <scope>...    Scope of current commit

ARGS:
    <message>    Use the given <message> as the commit message

```

## Notes

1. Relate one or more JIRA tasks：
    ```
    gic "hello, world" -j 100 -j 200 -s src/lib.rs -s src/main.rs
    ```

1. Change commit message：

    ```
    gic --amend --fix "get correct user count" -s src/members.rs --issue 123
    ```

# Dev

1. Don't forget to update the **version number** in `Cargo.toml`.
1. `make release`, generate executable file under `bin` directory.

# License

MIT License. Please feel free~