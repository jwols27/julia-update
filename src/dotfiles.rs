use crate::linha_comando::LinhaComando;
use chrono::Local;
use colored::Colorize;
use std::io;
use std::io::Write;
use std::path::PathBuf;

pub struct Dotfiles {
    git: PathBuf,
    args: Vec<String>,
}

impl Dotfiles {
    pub fn new(home: String) -> Self {
        let git_dir = format!("{}/.dotfiles", home);
        let work_tree = home;
        let args = vec![
            "--git-dir".to_owned(),
            git_dir,
            "--work-tree".to_owned(),
            work_tree,
        ];
        let git = which::which("git").expect("couldn't instantiate");
        Self { git, args }
    }

    pub fn salvar(&self) -> anyhow::Result<()> {
        print!("{}", " Mensagem do commit (opcional): ".magenta().bold());
        io::stdout().flush()?;

        let mut mensagem_commit = String::new();
        io::stdin()
            .read_line(&mut mensagem_commit)
            .expect("Erro ao ler");

        let mut commit = mensagem_commit.trim().to_string();
        if commit.is_empty() {
            commit = format!("Atualização {}", Local::now().format("%Y-%m-%d %T"));
        }

        self.command(&["add", "-u"])?;
        self.command(&["commit", "-m", &commit])?;
        self.command(&["push", "-u", "origin", "main"])?;
        println!("{}", " Enviado!".magenta().bold());

        Ok(())
    }

    pub fn command(&self, args: &[&str]) -> anyhow::Result<String> {
        let mut new_args = self.args.clone();
        new_args.extend(args.iter().map(|s| s.to_string()));

        LinhaComando::fresh(self.git.clone(), new_args).unsafe_get()
    }
}
