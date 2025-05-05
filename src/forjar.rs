use crate::{audio::reproduzir_audio, Alvo};

pub fn forjar(alvo: Alvo, conteudos: (&str, Option<String>)){
    match alvo{
        Alvo::Arquivo(caminho) => {
            match std::fs::write(&caminho, conteudos.0){
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
            caminho.push_str(&format!("\\{}", conteudos.0));
            let conteudo_arquivo = conteudos.1.unwrap_or("".to_string());
            if conteudo_arquivo.trim().is_empty(){
                return;
            }
            match std::fs::write(&caminho, conteudo_arquivo){
                Ok(()) => {
                    println!("Arquivo forjado com sucesso: {}", conteudos.0);
                },
                Err(e) => {
                    println!("ERRO: Não consegui forjar o arquivo {}: {}", conteudos.0, e);
                }
            }
        }
    }
    reproduzir_audio(crate::Comando::Forjar{caminho: String::new(), conteudo: String::new(), conteudo_arquivo: None});
}
