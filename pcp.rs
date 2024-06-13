use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::io::Read;

#[derive(Deserialize, Debug, PartialEq)]
pub struct MateriaPrima {
    pub nome: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Produto {
    pub nome: String,
    pub tempo_maquinario: u32, 
    pub materias_primas: Vec<(String, u32)>, 
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ItemPedido {
    pub nome: String,
    pub quantidade: u32,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Pedido {
    pub numero_pedido: u32,
    pub data_entrega: String, 
    pub itens: Vec<ItemPedido>,
}

#[derive(Deserialize)]
pub struct Limites {
    pub estoque: HashMap<String, (f64, f64)>, 
}


impl Limites {
    pub fn from_json<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = fs::File::open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let limites: Limites = serde_json::from_str(&contents)
            .map_err(|e| format!("Erro ao desserializar: {}", e))?;
        Ok(limites)
    }
}

impl MateriaPrima {
    pub fn from_json(file_name: &str) -> Vec<MateriaPrima> {
        let materias_primas = fs::read_to_string(file_name).expect("Erro ao ler arquivo");
        let materias_primas: Vec<MateriaPrima> = serde_json::from_str(&materias_primas).expect("Erro ao desserializar");
        materias_primas
    }
}

impl Produto {
    pub fn from_json(file_name: &str) -> Vec<Produto> {
        let produtos = fs::read_to_string(file_name).expect("Erro ao ler arquivo");
        let produtos: Vec<Produto> = serde_json::from_str(&produtos).expect("Erro ao desserializar");
        produtos
    }
}

impl Pedido {
    pub fn from_json(file_name: &str) -> Vec<Pedido> {
        let pedidos = fs::read_to_string(file_name).expect("Erro ao ler arquivo");
        let pedidos: Vec<Pedido> = serde_json::from_str(&pedidos).expect("Erro ao desserializar");
        pedidos
    }
}

pub fn otimizar_producao(pedidos: Vec<Pedido>, produtos: Vec<Produto>, limites: Limites) {
    let mut pedidos_ordenados = pedidos.clone();
    pedidos_ordenados.sort_by(|a, b| a.data_entrega.cmp(&b.data_entrega));

    let mut tempo_total_producao = 0.0;
    let mut pedidos_realizados: Vec<Pedido> = Vec::new();
    let mut pedidos_parcialmente_realizados: Vec<Pedido> = Vec::new();
    let mut materiais_necessarios: HashMap<String, f64> = HashMap::new();

    for pedido in &pedidos_ordenados {
        let mut pedido_parcial = pedido.clone();
        pedido_parcial.itens.clear();
        let mut pedido_completo = true;

        for item_pedido in &pedido.itens {
            if let Some(produto) = produtos.iter().find(|p| p.nome == item_pedido.nome) {
                let tempo_producao_item = produto.tempo_maquinario as f64 * item_pedido.quantidade as f64;
                
                if tempo_total_producao + tempo_producao_item <= 160.0 {
                    tempo_total_producao += tempo_producao_item;
                    pedido_parcial.itens.push(item_pedido.clone());

                    for materia in &produto.materias_primas {
                        *materiais_necessarios.entry(materia.0.clone())
                            .or_insert(0.0) += materia.1 as f64 * item_pedido.quantidade as f64;
                    }
                } else {
                    let quantidade_possivel = ((160.0 - tempo_total_producao) / produto.tempo_maquinario as f64).floor() as u32;
                    if quantidade_possivel > 0 {
                        let tempo_producao_parcial = produto.tempo_maquinario as f64 * quantidade_possivel as f64;
                        tempo_total_producao += tempo_producao_parcial;
                        
                        let mut item_parcial = item_pedido.clone();
                        item_parcial.quantidade = quantidade_possivel;
                        pedido_parcial.itens.push(item_parcial);

                        for materia in &produto.materias_primas {
                            *materiais_necessarios.entry(materia.0.clone())
                                .or_insert(0.0) += materia.1 as f64 * quantidade_possivel as f64;
                        }
                        pedido_completo = false;
                        break;
                    }
                }
            }
        }

        if !pedido_parcial.itens.is_empty() {
            if pedido_completo {
                pedidos_realizados.push(pedido.clone());
            } else {
                pedidos_parcialmente_realizados.push(pedido_parcial);
            }
        }
    }
    
    println!("Lista de compras de matérias-primas necessárias do dia:");
    println!("---------------------------------");
    let mut alguma_materia_comprada = false;
    for (nome, (quantidade_disponivel, capacidade_max)) in &limites.estoque {
        if let Some(&quantidade_necessaria) = materiais_necessarios.get(nome) {
            if *quantidade_disponivel == 0.0 || quantidade_necessaria > *capacidade_max {
                println!("Matéria-prima: {}, Quantidade a pedir: {:.2}", nome, quantidade_necessaria);
                alguma_materia_comprada = true;
            }
        } else {
            println!("Matéria-prima: {}, Quantidade a pedir: {:.2}", nome, quantidade_disponivel);
            alguma_materia_comprada = true;
        }
    }
    
    if !alguma_materia_comprada {
        println!("Nenhuma matéria-prima precisa ser comprada.");
    }
    println!("---------------------------------");
    println!("Pedidos a serem Produzidos no Dia:");
    println!("---------------------------------");
    for pedido in &pedidos_realizados {
        println!("Número do Pedido: {}", pedido.numero_pedido);
        println!("Data de Entrega: {}", pedido.data_entrega);
        for item_pedido in &pedido.itens {
            println!("Produto: {}, Quantidade: {}", item_pedido.nome, item_pedido.quantidade);
        }
        println!("---------------------------------");
    }

    for pedido_parcial in &pedidos_parcialmente_realizados {
        println!("Número do Pedido: {}", pedido_parcial.numero_pedido);
        println!("Data de Entrega: {}", pedido_parcial.data_entrega);
        for item_pedido in &pedido_parcial.itens {
            println!("Produto: {}, Quantidade Produzida: {}", item_pedido.nome, item_pedido.quantidade);
        }
        println!("Obs: Pedido parcialmente realizado.");
        println!("---------------------------------");
    }
}
