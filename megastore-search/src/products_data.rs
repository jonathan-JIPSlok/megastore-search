// src/products_data.rs
use crate::produto::Produto;

pub fn obter_produtos_exemplo() -> Vec<Produto> {
    vec![
        Produto::new(1, "Celular A".to_string(), "MarcaX".to_string(), "Eletrônicos".to_string(), 1200.0),
        Produto::new(2, "Celular B".to_string(), "MarcaY".to_string(), "Eletrônicos".to_string(), 1000.0),
        Produto::new(3, "Notebook A".to_string(), "MarcaX".to_string(), "Informática".to_string(), 3500.0),
        Produto::new(4, "Teclado Mecânico".to_string(), "MarcaZ".to_string(), "Informática".to_string(), 450.0),
        Produto::new(5, "Fone de Ouvido".to_string(), "MarcaY".to_string(), "Áudio".to_string(), 200.0),
        // ... (adicione mais produtos variadas aqui)
        // Para fins de exemplo, vou repetir produtos semelhantes para alcançar 100 itens
    ]
}
