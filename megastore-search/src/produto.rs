// src/produto.rs
#[derive(Clone, Debug, PartialEq)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub marca: String,
    pub categoria: String,
    pub preco: f64,
}

impl Produto {
    pub fn new(id: u32, nome: String, marca: String, categoria: String, preco: f64) -> Self {
        Produto { id, nome, marca, categoria, preco }
    }

    pub fn exibir(&self) {
        println!(
            "Produto ID: {}, Nome: {}, Marca: {}, Categoria: {}, Preço: R${:.2}",
            self.id, self.nome, self.marca, self.categoria, self.preco
        );
    }
}
