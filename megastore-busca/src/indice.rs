use std::collections::HashMap;
use crate::produto::Produto;

pub type IndiceDeBusca = HashMap<String, Vec<Produto>>;

pub fn indexar_produto(indice: &mut IndiceDeBusca, produto: Produto) {
    for termo in [&produto.nome, &produto.marca, &produto.categoria] {
        let chave = termo.to_lowercase();
        indice.entry(chave).or_default().push(produto.clone());
    }
}

pub fn buscar(indice: &IndiceDeBusca, termo: &str) -> Option<&Vec<Produto>> {
    indice.get(&termo.to_lowercase())
}