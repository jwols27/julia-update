use colored::Colorize;
use std::io;
use std::io::Write;

pub struct Pergunta {
    pergunta: String,
    opcoes: Vec<String>,
    padrao: String,
}

impl Pergunta {
    pub const SIM: &'static str = "s";
    pub const NAO: &'static str = "n";

    pub fn init(pergunta: &str, opcoes: &[&str], padrao: &str) -> Self {
        Self {
            pergunta: pergunta.to_owned(),
            opcoes: opcoes.iter().map(|s| s.to_ascii_lowercase()).collect(),
            padrao: padrao.to_owned(),
        }
    }

    // pub fn new(pergunta: &str, opcoes: &[&str], padrao: &str) -> String {
    //     Self::init(pergunta, opcoes, padrao).perguntar()
    // }

    pub fn com_nao(pergunta: &str, opcoes: &[&str], padrao: &str) -> String {
        let mut base = opcoes.to_vec();
        base.push(Self::NAO);
        Self::init(pergunta, &base, padrao).perguntar()
    }

    pub fn sim_nao(pergunta: &str) -> String {
        let opcoes: Vec<&str> = vec![Self::SIM, Self::NAO];
        Self::init(pergunta, &opcoes, Self::SIM).perguntar()
    }

    pub fn perguntar(&self) -> String {
        let escolhas = Self::listar(&self.opcoes, &self.padrao);

        print!("{} {} ", self.pergunta.magenta().bold(), escolhas.green());
        io::stdout().flush().expect("error");

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Erro ao ler");
        let input = buffer.trim().to_ascii_lowercase();

        for o in &self.opcoes {
            if o == &input {
                return o.to_owned();
            }
        }
        self.padrao.to_ascii_lowercase()
    }

    fn listar(opcoes: &Vec<String>, padrao: &str) -> String {
        let lista: Vec<String> = opcoes
            .iter()
            .map(|o| {
                if o == padrao {
                    o.to_ascii_uppercase()
                } else {
                    o.to_ascii_lowercase()
                }
            })
            .collect::<Vec<_>>();

        format!("[{}]", lista.join("/"))
    }
}
