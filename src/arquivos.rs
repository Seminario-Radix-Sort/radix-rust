use std::fs;
use chrono::Local;

#[allow(non_snake_case)]
pub fn leituraArquivo(vetor: &mut Vec<u64>, nomeArquivo: &mut String){
    let conteudo = fs::read_to_string(nomeArquivo)   
                        .expect("Erro ao ler o arquivo");

    let linhasArquivo = conteudo.lines();
    let linhaNumeros = linhasArquivo.last().expect("Linha de números do arquivo");

    for numero in linhaNumeros.split(", ").collect::<Vec<&str>>(){        
        vetor.push(
            numero.trim().parse::<u64>()
                .unwrap()
        );
    }
}

#[allow(non_snake_case)]
pub fn escritaArquivo(nomeArquivo: &mut String, tabelaSaida: &mut String){
    let dt = Local::now();
    let dataFormatada = dt.format("%d-%m-%Y-%H-%M-%S").to_string();

    nomeArquivo.push('-');
    nomeArquivo.push_str(&dataFormatada);
    nomeArquivo.push_str(".csv");

    let mut conteudo = String::from("Data e Hora de Geração: ".to_owned() + &dataFormatada + "\nTempos em segundos:\n");
    conteudo.push_str(tabelaSaida);

    fs::write(nomeArquivo, conteudo)
        .expect("Erro ao escrever o arquivo");
}