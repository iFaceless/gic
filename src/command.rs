use clap::{App, Arg, ArgMatches};

use crate::gitmoji::{Gitmoji, GitmojiConfig};

#[derive(Debug)]
pub struct CommandOption<'a> {
    pub scope: Vec<String>,
    pub jira: Vec<String>,
    pub issue: Vec<String>,
    pub message: String,
    pub amend: bool,
    pub gitmoji: Option<&'a Gitmoji>,
}

pub struct CommandApp<'a, 'b> {
    pub app: App<'a, 'b>,
    gitmoji_config: &'a GitmojiConfig,
}

impl<'a, 'b> CommandApp<'a, 'b> {
    pub fn new(conf: &'a GitmojiConfig) -> Self {
        let mut app = App::new("gic")
            .version("0.2.8")
            .about("Generate elegant and uniform commit messages~")
            .arg(
                Arg::with_name("scope")
                    .short("s")
                    .long("scope")
                    .help("Scope of current commit")
                    .multiple(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("jira")
                    .short("j")
                    .long("jira")
                    .help("Related JIRA task ID")
                    .multiple(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("issue")
                    .short("i")
                    .long("issue")
                    .help("Related issue ID")
                    .multiple(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("amend")
                    .short("a")
                    .long("amend")
                    .help("amend commit message"),
            )
            .arg(
                Arg::with_name("message")
                    .takes_value(true)
                    .required(true)
                    .index(1)
                    .help("Use the given <message> as the commit message"),
            );

        // 生成 gitmoji tags 命令行参数
        for it in conf.gitmojis.iter() {
            app = app.arg(
                Arg::with_name(&it.name)
                    .long(it.name.as_str())
                    .help(it.description.as_str()),
            );
        }

        CommandApp {
            app,
            gitmoji_config: conf,
        }
    }

    pub fn get_matches(&self) -> ArgMatches<'a> {
        self.app.to_owned().get_matches()
    }

    /// parse 用于解析用户输入的参数
    pub fn parse(&self, matches: &ArgMatches) -> CommandOption {
        let scope: Vec<&str> = match matches.values_of("scope") {
            Some(values) => values.collect(),
            None => Vec::new(),
        };
        let jira: Vec<&str> = match matches.values_of("jira") {
            Some(values) => values.collect(),
            None => Vec::new(),
        };
        let issue: Vec<&str> = match matches.values_of("issue") {
            Some(values) => values.collect(),
            None => Vec::new(),
        };
        let message = matches.value_of("message").unwrap();
        let mut found_gitmoji: Option<&Gitmoji> = None;

        for it in self.gitmoji_config.gitmojis.iter() {
            if matches.is_present(&it.name) {
                found_gitmoji = Some(it)
            }
        }
        CommandOption {
            jira: jira
                .iter()
                .filter(|&&x| !x.is_empty())
                .map(|x| x.to_string())
                .collect(),
            scope: scope
                .iter()
                .filter(|&&x| !x.is_empty())
                .map(|&x| x.to_string())
                .collect(),
            issue: issue
                .iter()
                .filter(|&&x| !x.is_empty())
                .map(|&x| x.to_string())
                .collect(),
            message: message.to_string(),
            gitmoji: found_gitmoji,
            amend: matches.is_present("amend"),
        }
    }
}
