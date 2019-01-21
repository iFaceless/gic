use gic::command::CommandApp;
use gic::commiter::Committer;
use gic::gitmoji::GitmojiConfig;

fn main() -> Result<(), std::io::Error> {
    let gitmoji_conf = GitmojiConfig::new();
    let app = CommandApp::new(&gitmoji_conf);
    let commiter = Committer::new(app.parse(&app.get_matches()));
    commiter.run()
}
