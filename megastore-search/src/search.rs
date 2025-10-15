use std::collections::HashMap;
use crate::product::Product;

/// Estrutura principal do mecanismo de busca da MegaStore.
#[derive(Debug)]
pub struct SearchEngine {
    index_name: HashMap<String, Vec<Product>>,
    index_brand: HashMap<String, Vec<Product>>,
    index_category: HashMap<String, Vec<Product>>,
}

impl SearchEngine {
    /// Cria uma nova instância do mecanismo de busca.
    pub fn new() -> Self {
        SearchEngine {
            index_name: HashMap::new(),
            index_brand: HashMap::new(),
            index_category: HashMap::new(),
        }
    }

    /// Adiciona um produto aos índices de nome, marca e categoria.
    pub fn add_product(&mut self, product: Product) {
        let name_key = product.name.to_lowercase();
        let brand_key = product.brand.to_lowercase();
        let category_key = product.category.to_lowercase();

        self.index_name
            .entry(name_key)
            .or_insert(Vec::new())
            .push(product.clone());

        self.index_brand
            .entry(brand_key)
            .or_insert(Vec::new())
            .push(product.clone());

        self.index_category
            .entry(category_key)
            .or_insert(Vec::new())
            .push(product);
    }

    /// Busca produtos pelo nome exato.
    pub fn search_by_name(&self, query: &str) -> Option<&Vec<Product>> {
        self.index_name.get(&query.to_lowercase())
    }

    /// Busca produtos pela marca exata.
    pub fn search_by_brand(&self, query: &str) -> Option<&Vec<Product>> {
        self.index_brand.get(&query.to_lowercase())
    }

    /// Busca produtos pela categoria exata.
    pub fn search_by_category(&self, query: &str) -> Option<&Vec<Product>> {
        self.index_category.get(&query.to_lowercase())
    }

    /// Busca produtos de forma parcial (nome, marca ou categoria contendo o termo).
    pub fn search_partial(&self, query: &str) -> Vec<&Product> {
        let q = query.to_lowercase();
        let mut results = Vec::new();

        for products in self.index_name.values() {
            for product in products {
                if product.name.to_lowercase().contains(&q)
                    || product.brand.to_lowercase().contains(&q)
                    || product.category.to_lowercase().contains(&q)
                {
                    results.push(product);
                }
            }
        }

        results
    }
}
