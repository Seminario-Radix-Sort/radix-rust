mod arquivos;
#[allow(non_snake_case)]
mod radixSort;

#[allow(non_snake_case)]
fn main() {
    let mut nomeArquivoSaida = "datasets/tempoExecucao-Rust".to_string();
    let mut tabelaSaida = String::new();

    println!("Calculando tabela de saída do Radix Sort...");
    radixSort::calcularTabelaSaidaRadix(vec![10000, 100000, 500000, 1000000], &mut tabelaSaida);    
    arquivos::escritaArquivo(&mut nomeArquivoSaida, &mut tabelaSaida);
    println!("Tabela de saída do Radix Sort calculada com sucesso!");
}
