use std::collections::HashMap;
use crate::produto::Produto;

#[derive(Default)]
pub struct Index {
    pub index_nome: HashMap<String, Vec<Produto>>,
    pub index_marca: HashMap<String, Vec<Produto>>,
    pub index_categoria: HashMap<String, Vec<Produto>>,
}

impl Index {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn adicionar_produto(&mut self, produto: Produto) {
        self.index_nome
            .entry(produto.nome.to_lowercase())
            .or_insert_with(Vec::new)
            .push(produto.clone());

        self.index_marca
            .entry(produto.marca.to_lowercase())
            .or_insert_with(Vec::new)
            .push(produto.clone());

        self.index_categoria
            .entry(produto.categoria.to_lowercase())
            .or_insert_with(Vec::new)
            .push(produto);
    }

    pub fn buscar_por_nome(&self, nome: &str) -> Option<&Vec<Produto>> {
        self.index_nome.get(&nome.to_lowercase())
    }

    pub fn buscar_por_marca(&self, marca: &str) -> Option<&Vec<Produto>> {
        self.index_marca.get(&marca.to_lowercase())
    }

    pub fn buscar_por_categoria(&self, categoria: &str) -> Option<&Vec<Produto>> {
        self.index_categoria.get(&categoria.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::produto::Produto;

    #[test]
    fn teste_adicionar_produto_ao_indice() {
        let mut indice = Index::new();
        let produto = Produto::new(1, "Produto1".to_string(), "Marca1".to_string(), "Categoria1".to_string(), 100.0);
        indice.adicionar_produto(produto.clone());

        let resultados = indice.buscar_por_nome("Produto1");
        assert!(resultados.is_some());
        let encontrados = resultados.unwrap();
        assert!(encontrados.contains(&produto));
    }

    #[test]
    fn teste_buscar_por_marca() {
        let mut indice = Index::new();
        let produto = Produto::new(2, "Produto2".to_string(), "MarcaX".to_string(), "Categoria2".to_string(), 200.0);
        indice.adicionar_produto(produto.clone());

        let resultados = indice.buscar_por_marca("MarcaX");
        assert!(resultados.is_some());
        assert!(resultados.unwrap().contains(&produto));
    }

    #[test]
    fn teste_buscar_por_categoria_nao_encontrado() {
        let indice = Index::new();
        let resultados = indice.buscar_por_categoria("CategoriaNaoExistente");
        assert!(resultados.is_none());
    }
}
