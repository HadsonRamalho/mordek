use util::{detectar_alvo, detectar_comando};
use agarrar::agarrar;
use forjar::forjar;
use obliterar::obliterar;

pub mod audio;
pub mod util;
pub mod agarrar;
pub mod forjar;
pub mod obliterar;

#[derive(Debug)]
pub enum Comando{
    Obliterar,
    Agarrar,
    Forjar,

    Help
}

#[derive(Debug)]
pub enum Alvo{
    Arquivo(String),
    Diretorio(String)
}

fn main() -> Result<(), String>{
    let comando = std::env::args().nth(1).expect("Nenhum comando fornecido");
    let alvo = std::env::args().nth(2).unwrap_or("".to_string());
    let padrao = std::env::args().nth(3).unwrap_or("".to_string());

    let comando = detectar_comando(&comando)?;
    match comando{
        Comando::Help => {
            listar_comandos();
            return Ok(());
        },
        _ => {}
    }

    let alvo = detectar_alvo(&alvo)?;

    match comando{
        Comando::Obliterar => {
            println!("{}", obliterar(alvo));
        },
        Comando::Agarrar => {
            agarrar(alvo, &padrao, false);
        },
        Comando::Forjar => {
            forjar(alvo, &padrao);
        },
        _ => {}
    }

    Ok(())
}

fn listar_comandos(){
    println!("Comandos disponíveis: obliterar, agarrar, forjar\n
    - Obliterar: Remove o alvo (arquivo ou diretório) do sistema.\n
    - Agarrar: Lê o conteúdo do alvo (arquivo ou diretório) e procura por um texto.\n
    - Forjar: Cria um novo arquivo ou diretório com o conteúdo especificado.\n\n
    
    Exemplo de uso:\n
    - --obliterar <caminho>\n
    - --agarrar <caminho> <texto>\n
    - --forjar <caminho> <conteúdo>\n");
}
