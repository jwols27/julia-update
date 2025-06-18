pub struct PerguntaOpcao {
    pub opcao: String,
}

impl PerguntaOpcao {
    pub const SIM: &'static str = "s";
    pub const NAO: &'static str = "n";

    pub fn new(opcao: &str) -> Self {
        Self {
            opcao: opcao.to_string().to_ascii_lowercase(),
        }
    }

    pub fn sim_nao() -> Vec<Self> {
        vec![Self::new(Self::SIM), Self::new(Self::NAO)]
    }

    pub fn com_nao(opcoes: &[&str]) -> Vec<Self> {
        let mut base: Vec<PerguntaOpcao> = Vec::new();
        for o in opcoes {
            base.push(PerguntaOpcao::new(o));
        }
        base.push(Self::new(Self::NAO));
        base
    }

    pub fn listar(base: &[Self], padrao: &str) -> String {
        let lista = base
            .iter()
            .map(|o| {
                if o.opcao == padrao {
                    o.opcao.to_ascii_uppercase()
                } else {
                    o.opcao.to_ascii_lowercase()
                }
            })
            .collect::<Vec<_>>();

        format!("[{}]", lista.join("/"))
    }
}
