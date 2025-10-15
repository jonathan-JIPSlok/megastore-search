use megastore_search::search::SearchEngine;
use megastore_search::product::Product;

/// Testa se o mecanismo de busca adiciona e encontra produtos corretamente.
#[test]
fn test_search_by_name() {
    let mut engine = SearchEngine::new();
    let p1 = Product::new(1, "iPhone 15", "Smartphone", "Apple");
    let p2 = Product::new(2, "Galaxy S24", "Smartphone", "Samsung");

    engine.add_product(p1.clone());
    engine.add_product(p2.clone());

    let results = engine.search_by_name("iPhone 15");
    assert!(results.is_some());
    assert_eq!(results.unwrap()[0].brand, "Apple");
}

/// Testa busca por marca exata.
#[test]
fn test_search_by_brand() {
    let mut engine = SearchEngine::new();
    let p1 = Product::new(1, "iPhone 15", "Smartphone", "Apple");
    let p2 = Product::new(2, "MacBook Air M2", "Notebook", "Apple");
    let p3 = Product::new(3, "Galaxy S24", "Smartphone", "Samsung");

    engine.add_product(p1);
    engine.add_product(p2);
    engine.add_product(p3);

    let results = engine.search_by_brand("Apple");
    assert!(results.is_some());
    let list = results.unwrap();
    assert_eq!(list.len(), 2);
}

/// Testa busca por categoria exata.
#[test]
fn test_search_by_category() {
    let mut engine = SearchEngine::new();
    let p1 = Product::new(1, "iPhone 15", "Smartphone", "Apple");
    let p2 = Product::new(2, "Galaxy S24", "Smartphone", "Samsung");
    let p3 = Product::new(3, "MacBook Air M2", "Notebook", "Apple");

    engine.add_product(p1);
    engine.add_product(p2);
    engine.add_product(p3);

    let results = engine.search_by_category("Smartphone");
    assert!(results.is_some());
    let list = results.unwrap();
    assert_eq!(list.len(), 2);
}

/// Testa busca parcial (nome, marca ou categoria contendo o termo).
#[test]
fn test_search_partial() {
    let mut engine = SearchEngine::new();
    let p1 = Product::new(1, "iPhone 15", "Smartphone", "Apple");
    let p2 = Product::new(2, "MacBook Air M2", "Notebook", "Apple");
    let p3 = Product::new(3, "Galaxy S24", "Smartphone", "Samsung");

    engine.add_product(p1);
    engine.add_product(p2);
    engine.add_product(p3);

    // Deve encontrar iPhone pelo termo "iphone"
    let results = engine.search_partial("iphone");
    assert_eq!(results.len(), 1);

    // Deve encontrar produtos da marca Apple
    let results = engine.search_partial("apple");
    assert_eq!(results.len(), 2);

    // Deve encontrar qualquer Smartphone
    let results = engine.search_partial("smart");
    assert_eq!(results.len(), 2);
}
