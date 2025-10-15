use std::io;
use megastore_search::search::SearchEngine;
use megastore_search::products_data::load_products;

fn main() {
    // Inicializa o mecanismo de busca
    let mut engine = SearchEngine::new();

    // Carrega os produtos pré-cadastrados
    let products = load_products();
    for product in products {
        engine.add_product(product);
    }

    // Interface inicial do sistema
    println!("Sistema de Busca MegaStore Iniciado!\n");
    println!("Selecione o tipo de busca:");
    println!("1 - Nome exato");
    println!("2 - Marca exata");
    println!("3 - Categoria exata");
    println!("4 - Busca parcial (qualquer termo)\n");

    // Lê a opção do usuário
    print!("Digite o número da opção desejada: ");
    let mut option = String::new();
    io::stdin().read_line(&mut option).unwrap();
    let option = option.trim();

    // Lê o termo de busca
    print!("Digite o termo de busca: ");
    let mut query = String::new();
    io::stdin().read_line(&mut query).unwrap();
    let query = query.trim();

    println!(); // quebra de linha para estética

    // Executa a busca conforme a opção escolhida
    match option {
        // Busca por nome exato
        "1" => match engine.search_by_name(query) {
            Some(results) => exibir_resultados(results),
            None => println!("Nenhum produto encontrado com esse nome."),
        },

        // Busca por marca exata
        "2" => match engine.search_by_brand(query) {
            Some(results) => exibir_resultados(results),
            None => println!("Nenhum produto encontrado com essa marca."),
        },

        // Busca por categoria exata
        "3" => match engine.search_by_category(query) {
            Some(results) => exibir_resultados(results),
            None => println!("Nenhum produto encontrado nessa categoria."),
        },

        // Busca parcial (por termo contido no nome, marca ou categoria)
        "4" => {
            let results = engine.search_partial(query);
            if results.is_empty() {
                println!("Nenhum produto encontrado para '{}'.", query);
            } else {
                println!("Resultados encontrados:");
                for product in results {
                    println!("{:?}", product);
                }
            }
        }

        // Caso o usuário insira uma opção inválida
        _ => println!("Opção inválida. Tente novamente."),
    }
}

/// Exibe a lista de produtos encontrados de forma padronizada.
fn exibir_resultados(results: &Vec<megastore_search::product::Product>) {
    println!("Resultados encontrados:");
    for product in results {
        println!("nome: {}", product.name);
        println!("Categoria: {}", product.category);
        println!("Marca: {}", product.brand);
        println!("", )
    }
}