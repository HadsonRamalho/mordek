use crate::{audio::reproduzir_audio, Alvo};

pub fn forjar(alvo: Alvo, conteudo: &str){
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
    reproduzir_audio(crate::Comando::Forjar);
}
