# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## Descrição do projeto
Este projeto implementa um sistema de busca otimizado para o catálogo de produtos da MegaStore. O sistema tem como objetivo fornecer buscas rápidas e precisas por nome, marca e categoria dos produtos cadastrados, garantindo uma experiência eficiente para os clientes. Com foco em desempenho e escalabilidade, o sistema utiliza estruturas de dados eficientes para suportar o crescimento contínuo do catálogo.

## Tecnologias utilizadas
- Linguagem Rust
- Biblioteca padrão Rust (`std::collections::HashMap`)
- Ferramenta de testes nativa do Rust (`cargo test`)
- Ferramentas de build e gerenciamento: `cargo`

## Instruções para execução do sistema de busca

1. Certifique-se que o Rust está instalado no seu ambiente. Caso não esteja, instale via [rustup](https://rustup.rs/).
2. Clone o repositório do projeto.
3. Navegue até o diretório do projeto.

Insira os seguintes códigos:
cargo build --release
cargo run

O comando `cargo run` compila e executa o sistema, que irá carregar o catálogo de produtos e realizar buscas de exemplo.

## Instruções para execução dos testes

Para executar os testes unitários e de integração, utilize o comando:
cargo test

Esse comando compilará e executará todos os testes definidos, validando as funcionalidades de busca e integridade dos dados.

## Exemplos de uso

O sistema oferece buscas por três campos principais:

- Busca por nome:
indice.buscar_por_nome("Nome do Produto");

- Busca por marca:
indice.buscar_por_marca("Marca do Produto");

- Busca por categoria:
indice.buscar_por_categoria("Categoria do Produto");

Retornando listas de produtos que correspondem exatamente ao termo informado, ou None caso não encontre resultados.

## Arquitetura do sistema

O sistema está organizado em módulos Rust com as seguintes responsabilidades:

- **produto.rs**: Define a estrutura `Produto` com atributos e métodos.
- **search.rs**: Implementa o índice hash para busca eficiente por campos.
- **products_data.rs**: Fornece dados exemplares para popular o catálogo.
- **main.rs**: Interface principal que coordena a inicialização e consultas.
- **lib.rs**: Raiz da biblioteca, exportando os módulos para reuso.

## Algoritmos e estruturas de dados utilizados

A base da busca é o uso de **tabelas hash** (`HashMap` do Rust) para indexar rapidamente produtos por nome, marca e categoria. Essa estrutura permite tempo constante \(O(1)\) para buscas exatas, utilizando funções de hash e encadeamento para armazenar múltiplos produtos com a mesma chave.

## Considerações sobre desempenho e escalabilidade

- O uso de índices hash no Rust garante buscas extremamente rápidas.
- O sistema escala de forma eficiente para grandes volumes de dados.
- O gerenciamento de memória e a segurança do Rust ajudam a evitar vazamentos e bugs.
- Testes mostram respostas instantâneas para cerca de 100 produtos, com capacidade para expansão.

## Contribuições

Contribuições são bem-vindas! Para contribuir:

1. Faça um fork deste repositório.
2. Crie uma branch para sua feature ou correção (`git checkout -b feature-nome`).
3. Faça commits claros e descritivos.
4. Envie um pull request detalhando as mudanças propostas.

Por favor, siga as boas práticas de código e escreva testes para novas funcionalidades.

## Licença

Este projeto está licenciado sob a licença MIT. Veja o arquivo LICENSE para mais detalhes.