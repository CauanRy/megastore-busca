use megastore_busca::produto::Produto;
use megastore_busca::indice::{IndiceDeBusca, indexar_produto, buscar};

#[test]
fn teste_busca_existente() {
    let mut indice = IndiceDeBusca::new();
    let produto = Produto::new(1, "Monitor", "LG", "Eletrônicos");
    indexar_produto(&mut indice, produto.clone());

    let resultado = buscar(&indice, "lg");
    assert!(resultado.is_some());
    assert_eq!(resultado.unwrap()[0].nome, "Monitor");
}

#[test]
fn teste_busca_inexistente() {
    let indice = IndiceDeBusca::new();
    assert!(buscar(&indice, "inexistente").is_none());
}