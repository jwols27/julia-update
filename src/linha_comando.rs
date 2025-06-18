use colored::Colorize;
use std::path::PathBuf;
use std::process::Command;
use which::which;

pub struct LinhaComando {
    comando: PathBuf,
    argumentos: Vec<String>,
}

impl LinhaComando {
    pub fn sudo(cmd: &str, args: &[&str]) -> Self {
        Self::instantiate(true, cmd, args)
    }
    pub fn new(cmd: &str, args: &[&str]) -> Self {
        Self::instantiate(false, cmd, args)
    }

    pub fn fresh(comando: PathBuf, argumentos: Vec<String>) -> Self {
        Self {
            comando,
            argumentos,
        }
    }

    fn instantiate(sudo: bool, cmd: &str, args: &[&str]) -> Self {
        let comando = if sudo { "sudo" } else { cmd }.to_owned();
        let mut argumentos: Vec<String> = Vec::new();
        if sudo {
            argumentos.push(
                which(cmd)
                    .expect("couldn't instantiate")
                    .to_str()
                    .expect("couldn't instantiate")
                    .to_owned(),
            );
        }
        argumentos.extend(args.iter().map(|s| s.to_string()));
        Self {
            comando: which(comando).expect("couldn't instantiate"),
            argumentos,
        }
    }

    pub fn execute(&mut self) -> anyhow::Result<()> {
        let comando = which(&self.comando)?;
        let args = &self.argumentos;

        let status = Command::new(comando).args(args).status()?;
        if !status.success() {
            eprintln!(
                "{} {}",
                " Erro ao executar:".red().bold(),
                status.code().unwrap(),
            );
            std::process::exit(1);
        }
        Ok(())
    }

    pub fn unsafe_get(&mut self) -> anyhow::Result<String> {
        let comando = which(&self.comando)?;
        let args = &self.argumentos;

        let output = Command::new(comando).args(args).output()?;
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(stdout)
    }
}
