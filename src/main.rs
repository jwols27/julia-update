mod linha_comando;
mod pergunta_opcao;

use crate::linha_comando::LinhaComando;
use crate::pergunta_opcao::PerguntaOpcao;
use anyhow::Result;
use chrono::Local;
use colored::Colorize;
use once_cell::sync::Lazy;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::{env, io};

static HOME: Lazy<String> = Lazy::new(|| env::var("HOME").expect("Erro ao buscar pasta Home"));

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let atalho = if args.len() > 1 { args[1].as_str() } else { "" };

    if atalho == "help" {
        print_help();
        return Ok(());
    }

    LinhaComando::new("clear", &[]).execute()?;
    gravar_pacotes()?;

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
        let resposta = perguntar_sim_nao(" Deseja remover todos os pacotes orfãos?");
        match resposta.as_str() {
            "s" => apagar_orfaos(orfaos.clone())?,
            _ => {}
        }
    }

    let resposta = perguntar_sim_nao("\n Deseja limpar o cache?");
    match resposta.as_str() {
        "s" => LinhaComando::new("paccache", &["-r"]).execute()?,
        _ => {}
    }

    print_julia("\n 5 de 5: Atualizando dotfiles");
    let dotfiles_status = dotfiles(&["status"])?;

    println!("{}", dotfiles_status);
    if dotfiles_status.contains("nothing to commit") {
        print_julia(" Nada a fazer");
    } else {
        let resposta = perguntar_sim_nao(" Deseja salvar alterações?");
        match resposta.as_str() {
            "s" => salvar_dotfiles()?,
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

    let opcoes: Vec<PerguntaOpcao> = PerguntaOpcao::com_nao(&["d", "r"]);
    let resposta = perguntar(
        "\n Deseja (d)esligar ou (r)einiciar o sistema?",
        opcoes,
        PerguntaOpcao::NAO,
    );
    match resposta.as_str() {
        "d" => LinhaComando::sudo("shutdown", &["now"]).execute()?,
        "r" => LinhaComando::sudo("reboot", &["now"]).execute()?,
        _ => {}
    }
    Ok(())
}

fn gravar_pacotes() -> Result<()> {
    let pacotes = LinhaComando::new("pacman", &["-Qqe"]).unsafe_get()?;

    let caminho_saida = format!("{}/Backups/pacotes.txt", *HOME);
    let caminho_ignorar = format!("{}/Backups/pacotes-ignorar.txt", *HOME);

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

fn perguntar_sim_nao<'a>(pergunta: &str) -> String {
    perguntar(pergunta, PerguntaOpcao::sim_nao(), PerguntaOpcao::SIM)
}

fn perguntar(pergunta: &str, opcoes: Vec<PerguntaOpcao>, padrao: &str) -> String {
    let escolhas = PerguntaOpcao::listar(&opcoes, &padrao);

    print!("{} {} ", pergunta.magenta().bold(), escolhas.green());
    io::stdout().flush().expect("error");

    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).expect("Erro ao ler");
    let input = buffer.trim().to_ascii_lowercase();

    for o in opcoes {
        if input == o.opcao {
            return o.opcao.clone();
        }
    }
    padrao.to_owned()
}

fn dotfiles(extra_args: &[&str]) -> Result<String> {
    let home = HOME.clone();
    let git_dir = format!("{}/.dotfiles", home);
    let work_tree = home;

    let mut args = vec![
        "--git-dir",
        git_dir.as_str(),
        "--work-tree",
        work_tree.as_str(),
    ];
    args.extend_from_slice(extra_args);

    let output = LinhaComando::new("git", &args).unsafe_get()?;

    Ok(output)
}

fn salvar_dotfiles() -> Result<()> {
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

    dotfiles(&["add", "-u"])?;
    dotfiles(&["commit", "-m", &commit])?;
    dotfiles(&["push", "-u", "origin", "main"])?;
    print_julia(" Enviado!");

    Ok(())
}

fn apagar_orfaos(orfaos: Vec<String>) -> Result<()> {
    let mut args: Vec<&str> = Vec::new();
    args.push("-Rns");
    args.extend(orfaos.iter().map(|s| s.as_str()));
    LinhaComando::new("yay", &args).execute()?;
    Ok(())
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
