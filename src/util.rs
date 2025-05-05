use crate::Alvo;


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
