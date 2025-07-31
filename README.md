# MegaStore - Sistema de Busca com Rust

## 📦 Visão Geral
Este projeto implementa um sistema de busca de produtos para uma loja virtual, utilizando **Rust** e a estrutura de dados `HashMap`.

A busca é baseada em palavras-chave (nome, marca, categoria) e retorna produtos correspondentes. 
É uma solução eficiente e escalável para grandes catálogos de e-commerce.

---

## 📁 Estrutura do Projeto
```
megastore-busca/
├── src/
│   ├── main.rs         // Ponto de entrada (CLI)
│   ├── produto.rs      // Estrutura Produto
│   └── indice.rs       // Indexador e busca
├── tests/
│   └── integracao.rs   // Testes automatizados
├── Cargo.toml
└── README.md
```

---

## 🚀 Como Rodar
1. Instale o Rust:
```bash
https://rustup.rs
```

2. Clone este repositório:
```bash
git clone https://github.com/seuusuario/megastore-busca
cd megastore-busca
```

3. Compile e execute o CLI:
```bash
cargo run
```

4. Execute os testes:
```bash
cargo test
```

---

## 🔍 Exemplo de Saída
```
Digite o termo de busca: acer
Resultados para 'acer':
Notebook Gamer (Acer) - Eletrônicos
```

---

## 🛠️ Tecnologias Usadas
- Linguagem: **Rust**
- Estrutura de dados: **HashMap** (tabela hash)
- Paradigma funcional e seguro

---

## 📚 Explicação Técnica
- `Produto`: Struct contendo `id`, `nome`, `marca` e `categoria`
- `IndiceDeBusca`: Mapeia palavras-chave para uma lista de produtos
- `indexar_produto()`: Insere produtos no índice com base em múltiplos termos
- `buscar()`: Retorna lista de produtos associados ao termo buscado
- `cargo test`: executa testes que verificam se o sistema responde corretamente a buscas

---

## ✅ Benefícios da Solução
- Busca rápida com complexidade O(1) na média
- Código seguro e sem vazamento de memória
- CLI simples para interações
- Fácil manutenção e expansão

---

## 👨‍💻 Autor
Cauan Santos de Oliveira  
[LinkedIn](https://www.linkedin.com/in/cauan-santos-de-oliveira/)
