use audio::obter_caminho_sistema;
use util::detectar_alvo;
use agarrar::agarrar;
use forjar::forjar;
use obliterar::obliterar;
use clap::{Parser, Subcommand};

pub mod audio;
pub mod util;
pub mod agarrar;
pub mod forjar;
pub mod obliterar;

/// Mordek: um utilitário de linha de comando para manipular arquivos com estilo.
#[derive(Parser, Debug)]
#[command(name = "mordek", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    comando: Comando,
}

#[derive(Subcommand, Debug, Default)]
pub enum Comando {
    /// Remove o alvo (arquivo ou diretório) do sistema
    Obliterar {
        /// Caminho do arquivo ou diretório
        caminho: String,
    },

    /// Lê o conteúdo do alvo (arquivo ou diretório) e procura por um texto
    Agarrar {
        /// Caminho do alvo
        caminho: String,
        /// Texto a ser procurado
        texto: String,
    },

    /// Cria um novo arquivo ou diretório com o conteúdo especificado
    Forjar {
        /// Caminho ou arquivo a ser criado
        caminho: String,
        /// Conteúdo da pasta ou arquivo (opcional)
        #[arg(default_value = "")]
        conteudo: String,
        /// Conteúdo do arquivo dentro da pasta (opcional)
        #[arg(default_value = "")]
        conteudo_arquivo: Option<String>,
    },

    /// Configura o idioma do sistema
    #[default]
    Setup,
}

#[derive(Debug)]
pub enum Alvo {
    Arquivo(String),
    Diretorio(String),
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.comando {
        Comando::Setup => {
            println!("Selecione um idioma: \n1. pt-BR\n2. en-US\n");
            let mut idioma = String::new();
            std::io::stdin()
                .read_line(&mut idioma)
                .expect("Erro ao ler o idioma.");
            let idioma = match idioma.trim() {
                "1" => "pt-BR",
                "2" => "en-US",
                _ => {
                    println!("Idioma inválido. O sistema usará o padrão (pt-BR).");
                    "pt-BR"
                }
            };
            let path = obter_caminho_sistema();
            let caminho_arquivo = if path.contains("/") {
                format!("{}/lang.mordek", path)
            } else {
                format!("{}\\lang.mordek", path)
            };
            std::fs::write(caminho_arquivo, idioma.trim())
                .expect("Erro ao salvar o idioma.");
            println!("Idioma salvo com sucesso.");
        }
        Comando::Obliterar { caminho } => {
            let alvo = detectar_alvo(&caminho)?;
            println!("{}", obliterar(alvo));
        }
        Comando::Agarrar { caminho, texto } => {
            let alvo = detectar_alvo(&caminho)?;
            agarrar(alvo, &texto, false);
        }
        Comando::Forjar { caminho, conteudo, conteudo_arquivo } => {
            let alvo = detectar_alvo(&caminho)?;
            forjar(alvo, (&conteudo, conteudo_arquivo));
        }
    }

    Ok(())
}
