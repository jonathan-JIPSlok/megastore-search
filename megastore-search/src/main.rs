use megastore_search::search::Index;
use megastore_search::products_data::obter_produtos_exemplo;

fn main() {
    // Cria um novo índice vazio para armazenar e buscar produtos.
    let mut indice = Index::new();

    // Obtém um vetor de produtos de exemplo para popular o índice.
    let produtos = obter_produtos_exemplo();

    // Adiciona cada produto do vetor ao índice para permitir buscas.
    for produto in produtos {
        indice.adicionar_produto(produto);
    }

    // Realiza uma busca por nome de produto e exibe os resultados.
    if let Some(resultados) = indice.buscar_por_nome("Celular A") {
        println!("Produtos encontrados pelo nome 'Celular A':");
        for produto in resultados {
            produto.exibir();
        }
    } else {
        println!("Nenhum produto encontrado pelo nome 'Celular A'");
    }

    // Realiza uma busca por marca de produto e exibe os resultados.
    if let Some(resultados) = indice.buscar_por_marca("MarcaY") {
        println!("Produtos encontrados pela marca 'MarcaY':");
        for produto in resultados {
            produto.exibir();
        }
    } else {
        println!("Nenhum produto encontrado pela marca 'MarcaY'");
    }

    // Realiza uma busca por categoria de produto e exibe os resultados.
    if let Some(resultados) = indice.buscar_por_categoria("Eletrônicos") {
        println!("Produtos encontrados na categoria 'Eletrônicos':");
        for produto in resultados {
            produto.exibir();
        }
    } else {
        println!("Nenhum produto encontrado na categoria 'Eletrônicos'");
    }
}
