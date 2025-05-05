use crate::{Alvo, Comando};


pub fn detectar_alvo(alvo: &str)
    -> Result<Alvo, String>{
    if alvo.trim().is_empty(){
        return Err("Alvo inválido.".to_string())
    }
    let alvo = if alvo.contains("."){
        Alvo::Arquivo(alvo.to_string())
    } else {
        Alvo::Diretorio(alvo.to_string())
    };

    return Ok(alvo)
}

pub fn detectar_comando(comando: &str)
    -> Result<Comando, String>{
    let res = match comando{
        "--obliterar" => {
            println!("Sucumba!");
            Comando::Obliterar
        },
        "--agarrar" => {
            println!("Nas minhas garras!");
            Comando::Agarrar
        }
        "--forjar" => {
            println!("Forjando...");
            Comando::Forjar
        },
        "--help" => {
            Comando::Help
        }
        "--setup" => {
            Comando::Setup
        },
        _ => {
            let erro = "Comando inválido. Use --help para ver os comandos disponíveis."; 
            return Err(erro.to_string())
        }
    };

    return Ok(res)
}