mod dotfiles;
mod linha_comando;
mod pergunta;

use crate::dotfiles::Dotfiles;
use crate::linha_comando::LinhaComando;
use crate::pergunta::Pergunta;
use anyhow::Result;
use colored::Colorize;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let atalho = if args.len() > 1 { args[1].as_str() } else { "" };

    if atalho == "help" {
        print_help();
        return Ok(());
    }

    let home = env::var("HOME")?;

    LinhaComando::new("clear", &[]).execute()?;
    gravar_pacotes(&home)?;

    print_julia(" 1 de 5: Atualizando repositório oficial");
    LinhaComando::sudo("pacman", &["-Syu"]).execute()?;

    print_julia("\n 2 de 5: Atualizando AUR");
    LinhaComando::new("yay", &["-Sua"]).execute()?;

    print_julia("\n 3 de 5: Atualizando Flatpaks");
    LinhaComando::new("flatpak", &["update"]).execute()?;

    print_julia("\n 4 de 5: Manutenção");
    print_julia(" Procurando pacotes órfãos...");
    let orfaos_output = LinhaComando::new("yay", &["-Qdtq"]).unsafe_get()?;
    let orfaos: Vec<String> = orfaos_output.lines().map(|s| s.to_string()).collect();
    let orfaos_qtd = orfaos.len();
    println!(
        "{} {}",
        " Pacotes orfãos encontrados:".magenta().bold(),
        if orfaos_qtd > 0 {
            orfaos_qtd.to_string().red().bold()
        } else {
            orfaos_qtd.to_string().green()
        },
    );
    if orfaos_qtd > 0 {
        let resposta = Pergunta::sim_nao(" Deseja remover todos os pacotes orfãos?");
        match resposta.as_str() {
            "s" => apagar_orfaos(orfaos.clone()),
            _ => {}
        }
    }

    let resposta = Pergunta::sim_nao("\n Deseja limpar o cache?");
    match resposta.as_str() {
        "s" => LinhaComando::new("paccache", &["-r"]).execute()?,
        _ => {}
    }

    print_julia("\n 5 de 5: Atualizando dotfiles");
    let dotfiles = Dotfiles::new(home);
    let dotfiles_status = dotfiles.command(&["status"])?;

    println!("{}", dotfiles_status);
    if dotfiles_status.contains("nothing to commit") {
        print_julia(" Nada a fazer");
    } else {
        let resposta = Pergunta::sim_nao(" Deseja salvar alterações?");
        match resposta.as_str() {
            "s" => dotfiles.salvar()?,
            _ => {}
        }
    }

    print_julia("\n Concluído! :3");

    match atalho {
        "reboot" => {
            LinhaComando::new("reboot", &[]).execute()?;
            return Ok(());
        }
        "shutdown" => {
            LinhaComando::new("shutdown", &["now"]).execute()?;
            return Ok(());
        }
        "skip" => return Ok(()),
        _ => {}
    }

    let resposta = Pergunta::com_nao(
        "\n Deseja (d)esligar ou (r)einiciar o sistema?",
        &["d", "r"],
        Pergunta::NAO,
    );
    match resposta.as_str() {
        "d" => LinhaComando::sudo("shutdown", &["now"]).execute()?,
        "r" => LinhaComando::sudo("reboot", &["now"]).execute()?,
        _ => {}
    }
    Ok(())
}

fn gravar_pacotes(home: &String) -> Result<()> {
    let pacotes = LinhaComando::new("pacman", &["-Qqe"]).unsafe_get()?;

    let caminho_saida = format!("{}/Backups/pacotes.txt", home);
    let caminho_ignorar = format!("{}/Backups/pacotes-ignorar.txt", home);

    let ignorar: Vec<String> = BufReader::new(File::open(caminho_ignorar)?)
        .lines()
        .collect::<Result<_, _>>()?;

    let mut writer = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(caminho_saida)?;

    for line in pacotes.lines() {
        if !ignorar.iter().any(|pattern| line.contains(pattern)) {
            writeln!(writer, "{}", line)?;
        }
    }

    Ok(())
}

fn apagar_orfaos(orfaos: Vec<String>) -> () {
    let mut args: Vec<&str> = vec!["-Rns"];
    args.extend(orfaos.iter().map(|s| s.as_str()));
    LinhaComando::new("yay", &args).execute().expect("error");
}

fn print_julia(msg: &str) {
    println!("{}", msg.magenta().bold());
}

fn print_help() {
    println!("Uso: julia-update [OPÇÃO]");
    println!("Atualiza o sistema faz uma manutenção básica.");
    println!("Opções disponíveis:");
    println!("  help\t\tMostra manual do comando.");
    println!("  reboot\tReinicia automaticamente no final.");
    println!("  skip\t\tExecuta o comando sem perguntar nada no final.");
    println!("  shutdown\tDesliga automaticamente no final.");
}
