use crate::command::CommandOption;
use std::process::Command;

/// Committer 维护了用于生成优雅的 Commit Message
/// 所需要的元信息
#[derive(Debug)]
pub struct Committer<'a> {
    opt: CommandOption<'a>,
}

impl<'a> Committer<'a> {
    pub fn new(opt: CommandOption<'a>) -> Self {
        Committer { opt }
    }

    pub fn run(&self) -> Result<(), std::io::Error> {
        let mut cmd = Command::new("git");
        let cmd = cmd.arg("commit").arg("-m").arg(self.build_commit_info());
        let cmd = if self.opt.amend {
            cmd.arg("--amend")
        } else {
            cmd
        };

        let output = cmd.output()?;
        if output.status.success() {
            let info = String::from_utf8_lossy(&output.stdout);
            if !info.is_empty() {
                println!("{}", info);
            }
        } else {
            let info = String::from_utf8_lossy(&output.stdout);
            if !info.is_empty() {
                println!("{}", info);
            }

            let error = String::from_utf8_lossy(&output.stderr);
            if !error.is_empty() {
                eprintln!("{}", error);
            }
        }
        Ok(())
    }

    fn build_commit_info(&self) -> String {
        if let Some(gitmoji) = self.opt.gitmoji {
            format!(
                "{emoji} {tag}{scope}: {message}{jira}{issue}",
                emoji = gitmoji.emoji,
                tag = gitmoji.name,
                message = &self.opt.message,
                scope = self.get_scope(),
                jira = self.get_jira(),
                issue = self.get_issue(),
            )
        } else {
            self.opt.message.to_owned()
        }
    }

    fn get_jira(&self) -> String {
        if self.opt.jira.is_empty() {
            return "".to_string();
        }

        let s: Vec<String> = self.opt.jira.iter().map(|x| format!("#{}", x)).collect();
        format!(" ({})", s.join(", "))
    }

    fn get_scope(&self) -> String {
        if self.opt.scope.is_empty() {
            return "".to_string();
        }

        format!("({})", self.opt.scope.join(", "))
    }

    fn get_issue(&self) -> String {
        if self.opt.issue.is_empty() {
            return "".to_string();
        }
        let s: Vec<String> = self
            .opt
            .issue
            .iter()
            .map(|x| format!("issue_{}", x))
            .collect();

        format!(" [{}]", s.join(", "))
    }
}
