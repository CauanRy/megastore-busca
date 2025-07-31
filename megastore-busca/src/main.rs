mod produto;
mod indice;

use std::io::{self, Write};
use produto::Produto;
use indice::{IndiceDeBusca, indexar_produto, buscar};

fn main() {
    let mut indice = IndiceDeBusca::new();

    let produtos = vec![
        Produto::new(1, "Notebook Gamer", "Acer", "Eletrônicos"),
        Produto::new(2, "Smartphone X", "TechBrand", "Celulares"),
        Produto::new(3, "Cafeteira Elétrica", "KitchenPro", "Eletrodomésticos"),
    ];

    for p in produtos.clone() {
        indexar_produto(&mut indice, p);
    }

    print!("Digite o termo de busca: ");
    io::stdout().flush().unwrap();
    let mut termo = String::new();
    io::stdin().read_line(&mut termo).unwrap();
    let termo = termo.trim();

    match buscar(&indice, termo) {
        Some(resultados) => {
            println!("Resultados para '{}':", termo);
            for r in resultados {
                println!("{} ({}) - {}", r.nome, r.marca, r.categoria);
            }
        }
        None => println!("Nenhum resultado encontrado para '{}'.", termo),
    }
}