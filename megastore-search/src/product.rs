// A macro `derive` adiciona automaticamente implementações de traits à estrutura.
// `Debug` permite imprimir a estrutura no console com `{:?}`.
// `Clone` permite criar cópias exatas de um objeto `Product`.
#[derive(Debug, Clone)]
pub struct Product {
    // Identificador único do produto
    pub id: u32,

    // Nome do produto (ex: "iPhone 15")
    pub name: String,

    // Categoria do produto (ex: "Smartphone", "Notebook")
    pub category: String,

    // Marca ou fabricante do produto (ex: "Apple", "Samsung")
    pub brand: String,
}

// Implementação dos métodos associados à estrutura `Product`
impl Product {
    /// Cria uma nova instância de `Product` com os dados informados.
    ///
    /// # Parâmetros
    /// * `id` - identificador numérico único do produto
    /// * `name` - nome do produto
    /// * `category` - categoria a que pertence o produto
    /// * `brand` - marca ou fabricante
    ///
    /// # Retorno
    /// Retorna um objeto `Product` completamente inicializado.
    pub fn new(id: u32, name: &str, category: &str, brand: &str) -> Self {
        Self {
            id,
            // Converte os parâmetros recebidos como `&str` para `String`,
            // pois os campos da struct armazenam valores em String.
            name: name.to_string(),
            category: category.to_string(),
            brand: brand.to_string(),
        }
    }
}

