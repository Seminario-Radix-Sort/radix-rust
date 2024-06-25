use chrono;
use crate::arquivos;

#[allow(non_snake_case)]
pub fn calcularTabelaSaidaRadix(tamanhos: Vec<i32>, tabelaSaida: &mut String) {
    let ordens = vec!["Aleatorio", "Crescente", "Decrescente"];
    let tipos = vec!["", "-RangeMenor", "-RangeMaior", "-CEP", "-Iguais", "-Extremo"];
    let mut nomeArquivo = String::new();
    tabelaSaida.push_str("Tamanho/Tipo,Aleatorio,Aleatorio-RangeMenor,Aleatorio-RangeMaior,Aleatorio-CEP,Aleatorio-Iguais,Aleatorio-Extremo,Crescente,Crescente-RangeMenor,Crescente-RangeMaior,Crescente-CEP,Crescente-Iguais,Crescente-Extremo,Decrescente,Decrescente-RangeMenor,Decrescente-RangeMaior,Decrescente-CEP,Decrescente-Iguais,Decrescente-Extremo\n");
    let mut tempoTotal: f64 = 0.0;
    let mut tempoMedio: f64 = 0.0;

    for tamanho in tamanhos {
        tabelaSaida.push_str(&tamanho.to_string());
        tabelaSaida.push(',');
        for ordem in &ordens {
            for tipo in &tipos {
                nomeArquivo.push_str("datasets/");
                nomeArquivo.push_str(&ordem.to_lowercase());
                nomeArquivo.push_str("s/");
                nomeArquivo.push_str(&tamanho.to_string());
                nomeArquivo.push_str(ordem);
                nomeArquivo.push_str(tipo);
                nomeArquivo.push_str(".txt");
                let mut vetor = Vec::<u64>::new();

                arquivos::leituraArquivo(&mut vetor, &mut nomeArquivo);

                for _ in 0..10 {
                    let mut vetorCopia = vetor.clone();
                    let inicio = chrono::Utc::now();
                    radixSort(&mut vetorCopia);
                    let fim = chrono::Utc::now();
                    let tempo = (fim - inicio).num_microseconds().unwrap() as f64 / 1_000_000.0;
                    tempoTotal += tempo as f64;
                    tempoMedio += tempo as f64;
                }
                let media: f64 = tempoMedio / 10.0;
                tabelaSaida.push_str(&format!("{:.8}", media.to_string()));
                tabelaSaida.push(',');
                tempoMedio = 0.0;

                nomeArquivo.clear();
            }
        }
        tabelaSaida.push('\n');
    }
    tabelaSaida.push_str("Tempo Total,");
    tabelaSaida.push_str(&format!("{:.10}", tempoTotal.to_string()));
    tabelaSaida.push('\n');
    tabelaSaida.push_str("Tempo Médio Total,");
    tabelaSaida.push_str(&format!("{:.8}", (tempoTotal / 720.0).to_string()));
    tabelaSaida.push('\n');
}

#[allow(non_snake_case)]
pub fn radixSort(vetor: &mut Vec<u64>){
    let maior = achaMaior(vetor);
    let mut exp: u64 = 1;

    while maior/exp > 0 {
        countingSort(vetor, exp);
        exp *= 10;
    }
}

#[allow(non_snake_case)]
pub fn achaMaior(vetor: &mut Vec<u64>) -> u64 {
    let mut maior: u64 = 0;
    for i in vetor {
        if *i > maior {
            maior = *i;
        }
    }
    maior
}

#[allow(non_snake_case)]
pub fn countingSort(vetor: &mut Vec<u64>, exp: u64) {
    let mut saida = vec![0; vetor.len()];
    let mut count = vec![0; 10];

    for i in &mut *vetor {
        count[((*i/exp) % 10) as usize] += 1;
    }

    for i in 1..10 {
        count[i] += count[i - 1];
    }

    for i in (0..vetor.len()).rev() {
        saida[count[((vetor[i]/exp) % 10) as usize] - 1] = vetor[i];
        count[((vetor[i]/exp) % 10) as usize] -= 1;
    }

    for i in 0..vetor.len() {
        vetor[i] = saida[i];
    }
}
