use rodio::{Decoder, OutputStream, source::Source};
use std::io::BufReader;
use std::{env, fs::File};

use crate::Comando;

pub fn audio_agarrar() -> (String, u64) {
    let rnd = rand::random_range(0..4);
    let info = match rnd {
        0 => ("nas_minhas_garras.mp3".to_string(), 3),
        1 => ("em_meus_dominios.mp3".to_string(), 4),
        2 => ("no_fim_tudo_sera_meu.mp3".to_string(), 5),
        _ => ("nas_minhas_garras.mp3".to_string(), 3),
    };

    info
}

pub fn obter_caminho_sistema() -> String{
    match env::current_exe() {
        Ok(path) => {
            if let Some(dir) = path.parent() {
                dir.to_str().unwrap().to_string()
            } else {
                println!("Erro ao obter path do executável.");
                return "".to_string()
            }
        }
        Err(e) => {
            eprintln!("Erro ao obter caminho do executável: {}", e);
            return "".to_string()
        }
    }
}

pub fn audio_obliterar() -> (String, u64) {
    let rnd = rand::random_range(0..4);
    let path = obter_caminho_sistema();
    let conteudo = std::fs::read_to_string(format!("{}\\lang.mordek", path)).unwrap();
    let info = match conteudo.as_str(){
        "pt-BR" => {
            match rnd {
                0 => ("sucumba.mp3".to_string(), 3),
                1 => ("aprecie_o_oblivio.mp3".to_string(), 4),
                2 => ("e_assim_comeca_o_massacre.mp3".to_string(), 4),
                _ => ("sucumba.mp3".to_string(), 3),
            }
        },
        _ => {
            ("aprecie_o_oblivio.mp3".to_string(), 4)
        }
    };

    info
}

pub fn audio_forjar() -> (String, u64) {
    let rnd = rand::random_range(0..4);
    let info = match rnd {
        0 => ("eu_curvei_o_reino_dos_mortos.mp3".to_string(), 10),
        _ => ("eu_curvei_o_reino_dos_mortos.mp3".to_string(), 10),
    };

    info
}

fn caminho_duracao_audio(caminho_sys: &str, info_audio: (String, u64)) -> (BufReader<File>, u64) {
    let caminho_arquivo = if caminho_sys.contains("/") {
        format!("{}/audios/{}", caminho_sys, info_audio.0)
    } else {
        format!("{}\\audios\\{}", caminho_sys, info_audio.0)
    };
    (
        BufReader::new(File::open(caminho_arquivo).unwrap()),
        info_audio.1,
    )
}

pub fn reproduzir_audio(comando: Comando) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let path = obter_caminho_sistema();

    let (file, duracao) = match comando {
        Comando::Obliterar => {
            let info = audio_obliterar();
            caminho_duracao_audio(&path, info)
        }
        Comando::Forjar => {
            let info = audio_forjar();

            caminho_duracao_audio(&path, info)
        }
        _ => {
            let info: (String, u64) = audio_agarrar();
            caminho_duracao_audio(&path, info)
        }
    };

    let source = Decoder::new(file).unwrap();
    let _ = stream_handle.play_raw(source.convert_samples());

    std::thread::sleep(std::time::Duration::from_secs(duracao));
}
