# TRABALHO DE OTIMIZAÇÃO

**Professor:** Alex Montanha  
**Aluno:** Lucas Ribeiro Moreira  
**RA:** 123112959

É com muito prazer que apresento o meu projeto de otimização e controle de sistema de produção.

## 1. O Problema

O problema em um termo geral consiste na otimização de um sistema de produção, especificamente maximizar a produção seguindo as restrições. O problema inclui logística de matérias-primas da compra e armazenamento, logística de produção como quantidades, ordem e tempo restrito, também logística de pedidos que basicamente consiste em conseguir produzir e entregar na data de entrega correta. O objetivo é conseguir dentro das restrições a melhor solução possível.

## 2. Classificação do Problema

O problema é classificado como NP, isso porque ele cresce em um tempo polinomial não determinístico, sendo mais claro quanto mais restrições ou entradas existirem mais o problema crescerá de forma não determinada.

## 3. Algoritmos Conhecidos

### Algoritmo de Planejamento de Produção Just-in-Time

**Objetivo do Just-in-Time:**

1. **Minimização de Estoques:** O principal objetivo do JIT é reduzir os níveis de estoque ao mínimo necessário, produzindo apenas o que é necessário, quando é necessário, e na quantidade necessária. Isso ajuda a minimizar os custos de armazenamento e evita o acúmulo de produtos obsoletos.
2. **Eficiência Operacional:** Ao sincronizar a produção com a demanda do cliente, o JIT melhora a eficiência operacional, reduzindo o desperdício de tempo e recursos.

**Características dos Algoritmos JIT:**

1. **Produção Puxada pela Demanda:** Em vez de produzir com base em previsões, o JIT produz com base em pedidos reais dos clientes. Isso significa que a produção é "puxada" pela demanda, não "empurrada" pela previsão.
2. **Tempo de Resposta Rápido:** Os sistemas JIT requerem tempos de resposta rápidos para alterações na demanda. Isso exige uma coordenação estreita com fornecedores e processos de produção flexíveis.
3. **Redução de Desperdícios:** Os algoritmos JIT focam na eliminação de qualquer forma de desperdício, incluindo tempo de espera, excesso de inventário, movimentação desnecessária, e defeitos nos produtos.
4. **Qualidade Consistente:** Para que o JIT funcione eficazmente, é essencial que os produtos sejam de alta qualidade e que os processos de produção sejam estáveis e controlados. Qualquer defeito pode interromper o fluxo de produção e causar atrasos.

### Material Requirements Planning (MRP)

**Objetivo do MRP:**

1. **Planejamento de Necessidades de Materiais:** O MRP visa garantir que os materiais e componentes estejam disponíveis para a produção e os produtos acabados estejam disponíveis para entrega aos clientes.
2. **Minimização de Estoques:** Embora o MRP mantenha estoques de segurança, ele também procura minimizar os níveis de estoque para reduzir custos.
3. **Programação de Produção:** O MRP ajuda a planejar as atividades de produção, entrega de materiais e agendamento de pedidos.

**Características dos Algoritmos MRP:**

1. **Planejamento Baseado na Demanda:** O MRP utiliza previsões de demanda e pedidos firmes para planejar as necessidades de materiais.
2. **Explosão da Lista de Materiais (BOM):** O MRP utiliza a lista de materiais (Bill of Materials - BOM) para determinar a quantidade de componentes e matérias-primas necessárias para produzir um produto final.
3. **Agendamento Retroativo:** O MRP trabalha retroativamente a partir da data de entrega desejada para determinar quando os materiais devem ser adquiridos e quando a produção deve começar.
4. **Níveis de Estoque de Segurança:** Mantém estoques de segurança para lidar com variações na demanda e atrasos na entrega de materiais.

## 4. Algoritmo Utilizado

O algoritmo utilizado é próprio, a partir do problema fui “desenhando” maneiras de resolver o problema da melhor forma, peço desculpas mas não entendi o que seria “implementar e versionar no GitHub”.

## 5. Complexidade do Algoritmo

O algoritmo utilizado possui uma complexidade de O(n log n), que é a maior complexidade do algoritmo, existente no trecho a seguir que é um algoritmo de ordenação (sort_by):

```rust
let mut pedidos_ordenados = pedidos.clone();
pedidos_ordenados.sort_by(|a, b| a.data_entrega.cmp(&b.data_entrega));
```
## 6. Evidenciar, analisar e apresentar quais estratégias o algoritmo escolhido utiliza

O algoritmo consiste em ordenar os pedidos por ordem de entrega para que os pedidos a serem entregues primeiro sejam os primeiros a serem fabricados, na parte do código a seguir:
```rust
let mut pedidos_ordenados = pedidos.clone();
pedidos_ordenados.sort_by(|a, b| a.data_entrega.cmp(&b.data_entrega));
````
Após ordenados, calcula o tempo de produção dos pedidos e seleciona na ordem todos que serão possíveis produzir (completo ou parcialmente) dentro da capacidade de produção diária, visto no trecho a seguir:
```rust
for item_pedido in &pedido.itens {
            if let Some(produto) = produtos.iter().find(|p| p.nome == item_pedido.nome) {
                let tempo_producao_item = produto.tempo_maquinario as f64 * item_pedido.quantidade as f64;
                
                if tempo_total_producao + tempo_producao_item <= 160.0 {
                    tempo_total_producao += tempo_producao_item;
                    pedido_parcial.itens.push(item_pedido.clone());

                    // Calcular o material necessário para a produção
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
```
Por fim, calcula quanto de matéria prima será necessário para a produção dos pedidos selecionados, visto no trecho a seguir:
```rust
for materia in &produto.materias_primas {
                            *materiais_necessarios.entry(materia.0.clone())
                                .or_insert(0.0) += materia.1 as f64 * quantidade_possivel as f64;
                        }
```
## 7. Comparação do Paradigma com o Algoritmo Just in Time

O algoritmo utilizado faz o processamento em lote, onde todos os pedidos são considerados e ordenados antes de qualquer produção ser iniciada. Já o Algoritmo JIT faz o processamento Just-in-Time, onde cada pedido é processado conforme chega, sem a necessidade de ordenar todos os pedidos previamente.

Em comparação de complexidade, o Algoritmo Just in Time possui uma menor complexidade, sendo O(n), exatamente por não ordenar os pedidos anteriormente.

## 8. Explicação do Algoritmo Desenvolvido em Rust

Vou explicar o algoritmo que desenvolvi em Rust para otimizar a produção de pedidos. O código lê dados de arquivos JSON, organiza esses dados em estruturas Rust e otimiza a produção com base nas limitações de tempo e materiais disponíveis.

Vou dividir a explicação em três partes principais:

1. **Leitura e Deserialização de Dados**: Como os dados são lidos de arquivos JSON e convertidos em estruturas Rust.
2. **Estruturação dos Dados**: Como organizei os dados e os preparei para o processamento.
3. **Processamento e Otimização da Produção**: A lógica que usei para calcular tempos de produção e necessidades de materiais, garantindo que os pedidos sejam realizados de maneira eficiente.

## Leitura e Deserialização de Dados
**Código:**
```rust
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::io::Read;

#[derive(Deserialize, Debug, PartialEq)]
pub struct MateriaPrima {
    pub nome: String,
    //pub tempo_entrega: u32
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Produto {
    pub nome: String,
    pub tempo_maquinario: u32, // Tempo de produção em horas
    pub materias_primas: Vec<(String, u32)>, // (nome do material, quantidade necessária)
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ItemPedido {
    pub nome: String,
    pub quantidade: u32,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Pedido {
    pub numero_pedido: u32,
    pub data_entrega: String, // Dia em que o pedido deve ser entregue
    pub itens: Vec<ItemPedido>,
}

#[derive(Deserialize)]
pub struct Limites {
    pub estoque: HashMap<String, (f64, f64)>, // (quantidade disponível, capacidade máxima)
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
```
## Explicação:

### 1. Uso de Crates Externas:
- **serde**: Utilizada para deserialização dos dados JSON para estruturas de dados Rust. A macro `#[derive(Deserialize)]` permite que as structs sejam automaticamente convertidas de JSON.

### 2. Módulo de Entrada/Saída:
- **std::fs**: Utilizado para operações de sistema de arquivos, como leitura de arquivos.
- **std::path::Path**: Facilita a manipulação de caminhos de arquivos.
- **std::io::Read**: Permite a leitura do conteúdo de arquivos.

### 3. Definição das Structs:
- **MateriaPrima, Produto, ItemPedido, Pedido, Limites**: Definidas para armazenar dados deserializados. Cada struct reflete a estrutura esperada dos dados JSON.

### 4. Métodos `from_json`:
- **Limites, MateriaPrima, Produto, Pedido**: Cada um implementa um método `from_json` que lê um arquivo JSON e desserializa seu conteúdo em instâncias das respectivas structs. Isso promove a reutilização de código e a separação de responsabilidades.

## Estrtuturas dos Dados
**Código:**
```rust
pub fn otimizar_producao(pedidos: Vec<Pedido>, produtos: Vec<Produto>, limites: Limites) {
    let mut pedidos_ordenados = pedidos.clone();
    pedidos_ordenados.sort_by(|a, b| a.data_entrega.cmp(&b.data_entrega));

    let mut tempo_total_producao = 0.0;
    let mut pedidos_realizados: Vec<Pedido> = Vec::new();
    let mut pedidos_parcialmente_realizados: Vec<Pedido> = Vec::new();
    let mut materiais_necessarios: HashMap<String, f64> = HashMap::new();
```
## Explicação:

### 1. Clonagem de Dados:
- **pedidos.clone()**: Cria uma cópia dos pedidos para preservação do estado original, utilizando a semântica de clonagem segura de Rust. Isso é importante para evitar modificações não intencionais nos dados originais.

### 2. Ordenação:
- **sort_by**: Utiliza uma comparação de datas (`data_entrega.cmp`) para ordenar os pedidos por data de entrega. A API de ordenação de Rust é poderosa e flexível, permitindo personalizações como essa.

### 3. Inicialização de Variáveis:
- Utilização de `Vec::new()` para inicialização de vetores e `HashMap::new()` para mapas hash. A tipagem explícita em `Vec<Pedido>` assegura clareza e robustez no código, facilitando a compreensão e manutenção.

## Processamento e Otimização da Produção
**Código:**
```rust
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
```
## Explicação:

### 1. Iteração e Clonagem:
- Uso de `for pedido in &pedidos_ordenados` para iterar sobre referências aos pedidos ordenados. Isso evita a movimentação dos dados e permite acesso eficiente.
- `pedido.clone()`: Cria uma cópia de cada pedido para manipulação segura.

### 2. Encontrar Produto Correspondente:
- `produtos.iter().find(|p| p.nome == item_pedido.nome)`: Utiliza iteradores e closures para encontrar o produto correspondente a cada item do pedido. Este padrão de busca é eficiente e idiomático em Rust.

### 3. Cálculos de Produção:
- Cálculo do tempo de produção total para cada item com `produto.tempo_maquinario as f64 * item_pedido.quantidade as f64`.
- Condicionais verificam se o tempo de produção excede o limite diário de 160.0 horas, ajustando a produção conforme necessário.

### 4. Atualização de HashMap:
- Uso do método `entry` para inserção ou atualização condicional de valores na HashMap.
- `or_insert(0.0)`: Define um valor padrão se a chave não existir, seguido de atualização direta usando desreferenciamento `*`.

### 5. Lógica de Produção Parcial:
- Cálculo da quantidade máxima possível para produção parcial com `quantidade_possivel`.
- Ajuste dos itens do pedido parcial com a quantidade possível e sinalização de pedidos parcialmente completos.

## Impressão dos Resultados
**Código:**
```rust
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
```
## Explicação:

### 1. Impressão Condicional:
- Utiliza `if let` para verificar a necessidade de matérias-primas e imprime a quantidade necessária.
- Mensagem específica é impressa se nenhuma matéria-prima precisar ser comprada.

### 2. Iteração e Impressão:
- Itera sobre `pedidos_realizados` e `pedidos_parcialmente_realizados`, imprimindo detalhes de cada pedido e itens produzidos.
- Utiliza formatadores `({:.2})` para controlar a precisão dos valores flutuantes na impressão.
