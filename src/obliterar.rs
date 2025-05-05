use crate::{audio::reproduzir_audio, Alvo, Comando};


pub fn obliterar(alvo: Alvo)
    -> String{
    match alvo{
        Alvo::Arquivo(caminho) => {
            match std::fs::remove_file(&caminho){
                Ok(()) => {
                    reproduzir_audio(Comando::Obliterar);
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
                    reproduzir_audio(Comando::Obliterar);
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