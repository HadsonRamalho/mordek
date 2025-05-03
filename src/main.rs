mod audio;
use audio::reproduzir_audio;

#[derive(Debug)]
pub enum Comando{
    Sucumbir,
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
        Comando::Sucumbir => {
            println!("{}", sucumbir(alvo));
        },
        Comando::Agarrar => {
            agarrar(alvo, &padrao);
        },
        Comando::Forjar => {
            forjar(alvo, &padrao);
        },
        _ => {}
    }

    Ok(())
}

fn listar_comandos(){
    println!("Comandos disponíveis: sucumbir, agarrar, forjar\n
    - Sucumbir: Remove o alvo (arquivo ou diretório) do sistema.\n
    - Agarrar: Lê o conteúdo do alvo (arquivo ou diretório) e procura por um texto.\n
    - Forjar: Cria um novo arquivo ou diretório com o conteúdo especificado.\n\n
    
    Exemplo de uso:\n
    - sucumbir <caminho>\n
    - agarrar <caminho> <texto>\n
    - forjar <caminho> <conteúdo>\n");
}

fn sucumbir(alvo: Alvo)
    -> String{
    match alvo{
        Alvo::Arquivo(caminho) => {
            match std::fs::remove_file(&caminho){
                Ok(()) => {
                    reproduzir_audio();
                    ()
                },
                Err(e) => {
                    match e.kind(){
                        std::io::ErrorKind::NotFound => {
                            return format!("ERRO: Parece que o arquivo {} já foi obliterado. Mas não por mim...", caminho);
                        },
                        _ => {
                            return format!("ERRO: Ainda não tenho poder suficiente para obliterar isso... {}", caminho);
                        }
                    }
                }
            }
        },
        Alvo::Diretorio(caminho) => {
            match std::fs::remove_dir_all(&caminho){
                Ok(()) => {
                    reproduzir_audio();
                    ()
                },
                Err(e) => {
                    match e.kind(){
                        std::io::ErrorKind::NotFound => {
                            return format!("ERRO: O diretório {} já foi obliterado. Mas não por mim...", caminho);
                        },
                        _ => {
                            return format!("ERRO: Ainda não tenho poder suficiente para obliterar isso... {}", caminho);
                        }
                    }
                }
            }
        }
    }
    return "O alvo foi lançado no esquecimento.".to_string()
}

fn agarrar(alvo: Alvo, padrao: &str){
    match alvo{
        Alvo::Arquivo(caminho) => {
            println!("Agarrando arquivo: {}", caminho);
            match std::fs::read_to_string(&caminho){
                Ok(conteudo) => {
                    for (num, line) in conteudo.lines().enumerate() {
                        if line.contains(&padrao) {
                            println!("Agarrei (na linha {}): \n\t{}\n", num + 1, line);
                        }
                    }
                },
                Err(e) => {
                    match e.kind(){
                        std::io::ErrorKind::NotFound => {
                            println!("ERRO: O arquivo {} escapou de minhas garras, mas caiu no esquecimento.", caminho);
                        },
                        _ => {
                            println!("ERRO: O arquivo {} está além do meu alcance... Por enquanto.", caminho);
                        }
                    }
                }
            }
        },
        Alvo::Diretorio(caminho) => {
            println!("Agarrando diretório: {}", caminho);
            match std::fs::read_dir(&caminho){
                Ok(conteudo) => {
                    for line in conteudo.into_iter(){
                        match line {
                            Ok(entry) => {
                                let path = entry.path();
                                if path.is_file() {
                                    match std::fs::read_to_string(&path){
                                        Ok(conteudo) => {
                                            for (num, line) in conteudo.lines().enumerate() {
                                                if !padrao.trim().is_empty() && line.contains(&padrao) {
                                                    println!("Agarrei ({:?} na linha {}): \n\t{}\n", path, num + 1, line);
                                                }
                                            }
                                        },
                                        Err(e) => {
                                            match e.kind(){
                                                std::io::ErrorKind::NotFound => {
                                                    println!("ERRO: O arquivo {} escapou de minhas garras, mas caiu no esquecimento.", path.display());
                                                },
                                                _ => {
                                                    println!("ERRO: O arquivo {} está além do meu alcance... Por enquanto.", path.display());
                                                }
                                            }
                                        }
                                    }
                                }
                                if path.is_dir(){
                                    agarrar(Alvo::Diretorio(entry.path().to_str().unwrap().to_string()), padrao);
                                }
                            },
                            Err(e) => {
                                println!("ERRO: Não consegui acessar o diretório {}: {}", caminho, e);
                            }
                        }
                    }
                },
                 Err(e) => {
                    println!("ERRO: Não consegui acessar o diretório {}: {}", caminho, e);
                }
            }
        }
    }
}

fn forjar(alvo: Alvo, conteudo: &str){
    match alvo{
        Alvo::Arquivo(caminho) => {
            match std::fs::write(&caminho, conteudo){
                Ok(()) => {
                    println!("Arquivo forjado com sucesso: {}", caminho);
                },
                Err(e) => {
                    println!("ERRO: Não consegui forjar o arquivo {}: {}", caminho, e);
                }
            }
        },
        Alvo::Diretorio(caminho) => {
            match std::fs::create_dir_all(&caminho){
                Ok(()) => {
                    println!("Diretório forjado com sucesso: {}", caminho);
                },
                Err(e) => {
                    println!("ERRO: Não consegui forjar o diretório {}: {}", caminho, e);
                    return;
                }
            }
            let mut caminho = caminho.clone();
            caminho.push_str(&format!("\\{}", conteudo));
            let conteudo_arquivo = std::env::args().nth(4).unwrap_or("".to_string());
            match std::fs::write(&caminho, conteudo_arquivo){
                Ok(()) => {
                    println!("Arquivo forjado com sucesso: {}", conteudo);
                },
                Err(e) => {
                    println!("ERRO: Não consegui forjar o arquivo {}: {}", conteudo, e);
                }
            }
        }
    }
}

fn detectar_alvo(alvo: &str)
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

fn detectar_comando(comando: &str)
    -> Result<Comando, String>{
    let res = match comando{
        "obliterar" => {
            println!("Sucumba!");
            Comando::Sucumbir
        },
        "agarrar" => {
            println!("Nas minhas garras!");
            Comando::Agarrar
        }
        "forjar" => {
            println!("Forjando...");
            Comando::Forjar
        },
        "help" => {
            Comando::Help
        }
        _ => {
            let erro = "Comando inválido. Use --help para ver os comandos disponíveis."; 
            return Err(erro.to_string())
        }
    };

    return Ok(res)
}