#[derive(Debug, Clone)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub marca: String,
    pub categoria: String,
}

impl Produto {
    pub fn new(id: u32, nome: &str, marca: &str, categoria: &str) -> Self {
        Produto {
            id,
            nome: nome.to_string(),
            marca: marca.to_string(),
            categoria: categoria.to_string(),
        }
    }
}