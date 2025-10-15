/// Representa um produto no catálogo da MegaStore.
///
/// Contém os atributos básicos necessários para identificar um produto e seus dados relevantes.
#[derive(Clone, Debug, PartialEq)]
pub struct Produto {
    /// Identificador único do produto.
    pub id: u32,
    /// Nome do produto.
    pub nome: String,
    /// Marca do produto.
    pub marca: String,
    /// Categoria em que o produto se enquadra.
    pub categoria: String,
    /// Preço do produto.
    pub preco: f64,
}

impl Produto {
    /// Cria uma nova instância de Produto com os dados fornecidos.
    ///
    /// # Argumentos
    ///
    /// * `id` - Número único identificador do produto.
    /// * `nome` - Nome do produto.
    /// * `marca` - Marca associada ao produto.
    /// * `categoria` - Categoria à qual o produto pertence.
    /// * `preco` - Preço de venda do produto.
    ///
    /// # Retorna
    ///
    /// Uma instância `Produto` com os valores inicializados.
    pub fn new(id: u32, nome: String, marca: String, categoria: String, preco: f64) -> Self {
        Produto { id, nome, marca, categoria, preco }
    }

    /// Exibe as informações do produto formatadas para o console.
    ///
    /// Mostra os valores de id, nome, marca, categoria e preço com duas casas decimais.
    pub fn exibir(&self) {
        println!(
            "Produto ID: {}, Nome: {}, Marca: {}, Categoria: {}, Preço: R${:.2}",
            self.id, self.nome, self.marca, self.categoria, self.preco
        );
    }
}
