mod pcp;


fn main() {
    let limites = match pcp::Limites::from_json("data/limites.json") {
        Ok(limites) => limites,
        Err(e) => {
            eprintln!("Erro ao ler o arquivo: {}", e);
            return;
        }
    };


    let _materias_primas = pcp::MateriaPrima::from_json("data/materias_primas.json");
    let produtos = pcp::Produto::from_json("data/produtos.json");
    let pedidos = pcp::Pedido::from_json("data/pedidos.json");

    pcp::otimizar_producao(pedidos, produtos, limites);
}
