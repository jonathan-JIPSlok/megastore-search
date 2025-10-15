extern crate megastore_search;

use megastore_search::produto::Produto;
use megastore_search::search::Index;

#[test]
fn teste_adicionar_e_buscar_por_nome() {
    let mut index = Index::new();

    let produto1 = Produto::new(1, "Celular".to_string(), "MarcaA".to_string(), "Eletrônicos".to_string(), 1500.0);
    let produto2 = Produto::new(2, "Celular".to_string(), "MarcaB".to_string(), "Eletrônicos".to_string(), 1200.0);
    let produto3 = Produto::new(3, "Notebook".to_string(), "MarcaA".to_string(), "Informática".to_string(), 3500.0);

    index.adicionar_produto(produto1.clone());
    index.adicionar_produto(produto2.clone());
    index.adicionar_produto(produto3.clone());

    let resultados = index.buscar_por_nome("Celular").expect("Deve encontrar produtos com nome Celular");
    assert_eq!(resultados.len(), 2);
    assert!(resultados.contains(&&produto1));
    assert!(resultados.contains(&&produto2));
}

#[test]
fn teste_buscar_por_marca() {
    let mut index = Index::new();

    let produto1 = Produto::new(1, "Celular".to_string(), "MarcaA".to_string(), "Eletrônicos".to_string(), 1500.0);
    let produto2 = Produto::new(2, "Fone".to_string(), "MarcaB".to_string(), "Eletrônicos".to_string(), 200.0);

    index.adicionar_produto(produto1.clone());
    index.adicionar_produto(produto2.clone());

    let resultados = index.buscar_por_marca("MarcaB").expect("Deve encontrar produtos da MarcaB");
    assert_eq!(resultados.len(), 1);
    assert_eq!(resultados[0].id, produto2.id);
}

#[test]
fn teste_buscar_por_categoria() {
    let mut index = Index::new();

    let produto1 = Produto::new(1, "Celular".to_string(), "MarcaA".to_string(), "Eletrônicos".to_string(), 1500.0);
    let produto2 = Produto::new(2, "Cadeira".to_string(), "MarcaC".to_string(), "Móveis".to_string(), 450.0);

    index.adicionar_produto(produto1.clone());
    index.adicionar_produto(produto2.clone());

    let resultados = index.buscar_por_categoria("Eletrônicos").expect("Deve encontrar produtos da categoria Eletrônicos");
    assert_eq!(resultados.len(), 1);
    assert_eq!(resultados[0].id, produto1.id);
}
