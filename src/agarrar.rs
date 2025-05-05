use crate::{audio::reproduzir_audio, Alvo, Comando};

pub fn agarrar(alvo: Alvo, padrao: &str, audio_tocou: bool){
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
                            // println!("ERRO: O arquivo {} escapou de minhas garras, mas caiu no esquecimento.", caminho);
                        },
                        _ => {
                            // println!("ERRO: O arquivo {} está além do meu alcance... Por enquanto.", caminho);
                        }
                    }
                }
            }
        },
        Alvo::Diretorio(caminho) => {
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
                                                    // println!("ERRO: O arquivo {} escapou de minhas garras, mas caiu no esquecimento.", path.display());
                                                },
                                                _ => {
                                                    // println!("ERRO: O arquivo {} está além do meu alcance... Por enquanto.", path.display());
                                                }
                                            }
                                        }
                                    }
                                }
                                if path.is_dir(){
                                    agarrar(Alvo::Diretorio(entry.path().to_str().unwrap().to_string()), padrao, true);
                                }
                            },
                            Err(e) => {
                                // println!("ERRO: Não consegui acessar o diretório {}: {}", caminho, e);
                            }
                        }
                    }
                },
                 Err(e) => {
                    // println!("ERRO: Não consegui acessar o diretório {}: {}", caminho, e);
                }
            }
        }
    }
    if !audio_tocou{
        reproduzir_audio(Comando::Agarrar{caminho: String::new(), texto: String::new()});
    }
}
