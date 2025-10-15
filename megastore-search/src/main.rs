// src/main.rs

use megastore_search::produto::Produto;
use megastore_search::search::Index;
use megastore_search::products_data::obter_produtos_exemplo;

fn main() {
    let mut indice = Index::new();

    // Obter produtos exemplo
    let produtos = obter_produtos_exemplo();

    // Adicionar produtos ao índice
    for produto in produtos {
        indice.adicionar_produto(produto);
    }

    // Exemplo de busca por nome
    if let Some(resultados) = indice.buscar_por_nome("Celular A") {
        println!("Produtos encontrados pelo nome 'Celular A':");
        for produto in resultados {
            produto.exibir();
        }
    } else {
        println!("Nenhum produto encontrado pelo nome 'Celular A'");
    }

    // Exemplo de busca por marca
    if let Some(resultados) = indice.buscar_por_marca("MarcaY") {
        println!("Produtos encontrados pela marca 'MarcaY':");
        for produto in resultados {
            produto.exibir();
        }
    } else {
        println!("Nenhum produto encontrado pela marca 'MarcaY'");
    }

    // Exemplo de busca por categoria
    if let Some(resultados) = indice.buscar_por_categoria("Eletrônicos") {
        println!("Produtos encontrados na categoria 'Eletrônicos':");
        for produto in resultados {
            produto.exibir();
        }
    } else {
        println!("Nenhum produto encontrado na categoria 'Eletrônicos'");
    }
}
