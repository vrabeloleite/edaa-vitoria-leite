use std::collections::{HashMap, VecDeque};

// =========================================================================
// ESTRUTURAS DE DADOS EXIGIDAS PELOS GRUPOS
// =========================================================================

// Grupo 2 - Exercício 6
pub struct Navegador {
    pub atual: String,
    pub historico_back: Vec<String>,
    pub historico_forward: Vec<String>,
}

impl Navegador {
    pub fn new(home: &str) -> Self {
        Self {
            atual: home.to_string(),
            historico_back: Vec::new(),
            historico_forward: Vec::new(),
        }
    }
    pub fn visitar(&mut self, nova_url: &str) {
        self.historico_back.push(self.atual.clone());
        self.atual = nova_url.to_string();
        self.historico_forward.clear();
    }
    pub fn voltar(&mut self) {
        if let Some(url) = self.historico_back.pop() {
            self.historico_forward.push(self.atual.clone());
            self.atual = url;
        }
    }
    pub fn avancar(&mut self) {
        if let Some(url) = self.historico_forward.pop() {
            self.historico_back.push(self.atual.clone());
            self.atual = url;
        }
    }
}

// Grupo 2 - Exercício 7
pub struct EditorTexto {
    pub texto: String,
    pub pilha_desfazer: Vec<String>,
    pub pilha_refazer: Vec<String>,
}

impl EditorTexto {
    pub fn new() -> Self {
        Self {
            texto: String::new(),
            pilha_desfazer: Vec::new(),
            pilha_refazer: Vec::new(),
        }
    }
    pub fn digitar(&mut self, novo_texto: &str) {
        self.pilha_desfazer.push(self.texto.clone());
        self.texto.push_str(novo_texto);
        self.pilha_refazer.clear();
    }
    pub fn desfazer(&mut self) {
        if let Some(estado_anterior) = self.pilha_desfazer.pop() {
            self.pilha_refazer.push(self.texto.clone());
            self.texto = estado_anterior;
        }
    }
    pub fn refazer(&mut self) {
        if let Some(estado_proximo) = self.pilha_refazer.pop() {
            self.pilha_desfazer.push(self.texto.clone());
            self.texto = estado_proximo;
        }
    }
}

// Grupo 2 - Exercício 9
pub struct StackMin {
    pub pilha_principal: Vec<i32>,
    pub pilha_min: Vec<i32>,
}

impl StackMin {
    pub fn new() -> Self {
        Self {
            pilha_principal: Vec::new(),
            pilha_min: Vec::new(),
        }
    }
    pub fn push(&mut self, val: i32) {
        self.pilha_principal.push(val);
        let atual_min = self.pilha_min.last().copied().unwrap_or(val);
        self.pilha_min.push(std::cmp::min(val, atual_min));
    }
    pub fn pop(&mut self) -> Option<i32> {
        self.pilha_min.pop();
        self.pilha_principal.pop()
    }
    pub fn min(&self) -> Option<i32> {
        self.pilha_min.last().copied()
    }
}

// Grupo 3 - Exercício 10
pub struct Cliente {
    pub id: usize,
    pub tempo_chegada: u32,
}

// Grupo 3 - Exercício 11
pub struct TrabalhoImpressao {
    pub nome: String,
    pub paginas: u32,
}

// Grupo 3 - Exercício 12
pub struct FilaCircular {
    pub buffer: Vec<Option<String>>,
    pub capacidade: usize,
    pub inicio: usize,
    pub fim: usize,
}

impl FilaCircular {
    pub fn new(capacidade: usize) -> Self {
        Self {
            buffer: vec![None; capacidade],
            capacidade,
            inicio: 0,
            fim: 0,
        }
    }
    pub fn enqueue(&mut self, msg: String) {
        if self.buffer[self.fim].is_some() {
            self.inicio = (self.inicio + 1) % self.capacidade;
        }
        self.buffer[self.fim] = Some(msg);
        self.fim = (self.fim + 1) % self.capacidade;
    }
    pub fn dequeue(&mut self) -> Option<String> {
        if self.buffer[self.inicio].is_none() {
            return None;
        }
        let msg = self.buffer[self.inicio].take();
        self.inicio = (self.inicio + 1) % self.capacidade;
        msg
    }
}

// Grupo 3 - Exercício 13
pub struct ItemPrioridade {
    pub dado: String,
    pub prioridade: i32,
}
pub struct FilaPrioridadeManual {
    pub itens: Vec<ItemPrioridade>,
}

impl FilaPrioridadeManual {
    pub fn new() -> Self {
        Self { itens: Vec::new() }
    }
    pub fn enqueue(&mut self, item: ItemPrioridade) {
        self.itens.push(item);
    }
    pub fn dequeue(&mut self) -> Option<ItemPrioridade> {
        if self.itens.is_empty() {
            return None;
        }
        let mut indice_maior = 0;
        for i in 1..self.itens.len() {
            if self.itens[i].prioridade > self.itens[indice_maior].prioridade {
                indice_maior = i;
            }
        }
        Some(self.itens.remove(indice_maior))
    }
}

// Grupo 4 - Exercício 16
#[derive(Debug)]
pub struct Tarefa {
    pub nome: String,
}
pub struct FilaTarefas {
    pub deque: VecDeque<Tarefa>,
}

impl FilaTarefas {
    pub fn new() -> Self {
        Self {
            deque: VecDeque::new(),
        }
    }
    pub fn adicionar_urgente(&mut self, t: Tarefa) {
        self.deque.push_front(t);
    }
    pub fn adicionar_normal(&mut self, t: Tarefa) {
        self.deque.push_back(t);
    }
    pub fn processar_proxima(&mut self) -> Option<Tarefa> {
        self.deque.pop_front()
    }
}

// Grupo 5 - Exercício 20
#[derive(Clone, Debug)]
pub struct Processo {
    pub id: usize,
    pub tempo_restante: u32,
}

// =========================================================================
// FUNÇÕES INDIVIDUAIS COM ANÁLISE DE COMPLEXIDADE
// =========================================================================

/// 1. Inversão com Vec
/// Tempo: O(n) | Espaço: O(n)
pub fn inverter_vetor(mut vec: Vec<i32>) -> Vec<i32> {
    let mut aux = Vec::new();
    while let Some(elemento) = vec.pop() {
        aux.push(elemento);
    }
    aux
}

/// 2. Contador de ocorrências
/// Tempo: O(n) | Espaço: O(k)
pub fn contar_ocorrencias(vec: Vec<char>) -> HashMap<char, usize> {
    let mut contador = HashMap::new();
    for x in &vec {
        *contador.entry(*x).or_insert(0) += 1;
    }
    contador
}

/// 3. Remoção condicional de pares
/// Tempo: O(n) | Espaço: O(n)
pub fn remover_pares(vec: Vec<i32>) -> Vec<i32> {
    let mut impares = Vec::new();
    for num in vec {
        if num % 2 != 0 {
            impares.push(num);
        }
    }
    impares
}

/// 4. Mescla ordenada manual (Two-Pointers)
/// Tempo: O(n + m) | Espaço: O(n + m)
pub fn mescla_ordenada(vec1: &[i32], vec2: &[i32]) -> Vec<i32> {
    let mut resultado = Vec::with_capacity(vec1.len() + vec2.len());
    let (mut i, mut j) = (0, 0);
    while i < vec1.len() && j < vec2.len() {
        if vec1[i] <= vec2[j] {
            resultado.push(vec1[i]);
            i += 1;
        } else {
            resultado.push(vec2[j]);
            j += 1;
        }
    }
    resultado.extend_from_slice(&vec1[i..]);
    resultado.extend_from_slice(&vec2[j..]);
    resultado
}

/// 5. Calculadora RPN
/// Tempo: O(n) | Espaço: O(n)
pub fn avaliar_rpn(expressao: &str) -> f64 {
    let mut p: Vec<f64> = Vec::new();
    for token in expressao.split_whitespace() {
        match token {
            "+" | "-" | "*" | "/" => {
                if let (Some(b), Some(a)) = (p.pop(), p.pop()) {
                    let res = match token {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        "/" => a / b,
                        _ => 0.0,
                    };
                    p.push(res);
                }
            }
            num => {
                if let Ok(val) = num.parse::<f64>() {
                    p.push(val);
                }
            }
        }
    }
    p.pop().unwrap_or(0.0)
}

/// 8. Sequências de símbolos (Delimitadores Balanceados)
/// Tempo: O(n) | Espaço: O(n)
pub fn delimitadores_balanceados(expr: &str) -> bool {
    let mut pilha = Vec::new();
    for c in expr.chars() {
        match c {
            '(' | '[' | '{' => pilha.push(c),
            ')' => { if pilha.pop() != Some('(') { return false; } }
            ']' => { if pilha.pop() != Some('[') { return false; } }
            '}' => { if pilha.pop() != Some('{') { return false; } }
            _ => {}
        }
    }
    pilha.is_empty()
}

/// 10. Simulador de Fila de Banco
/// Tempo: O(n) | Espaço: O(n)
pub fn calcular_espera_banco(clientes: Vec<Cliente>, tempo_atendimento: u32) -> f32 {
    let mut fila = VecDeque::from(clientes);
    let mut tempo_total_espera = 0;
    let mut total_clientes = 0;
    let mut tempo_atual = 0;

    while let Some(cliente) = fila.pop_front() {
        if tempo_atual < cliente.tempo_chegada {
            tempo_atual = cliente.tempo_chegada;
        }
        tempo_total_espera += tempo_atual - cliente.tempo_chegada;
        total_clientes += 1;
        tempo_atual += tempo_atendimento;
    }
    if total_clientes == 0 { 0.0 } else { tempo_total_espera as f32 / total_clientes as f32 }
}

/// 11. Impressora Compartilhada
/// Tempo: O(n) | Espaço: O(n)
pub fn simular_impressora(mut fila: VecDeque<TrabalhoImpressao>) {
    while let Some(t) = fila.pop_front() {
        println!("  [Impressora] Processado: {} ({} págs)", t.nome, t.paginas);
    }
}

/// 14. Palíndromo com Deque
/// Tempo: O(n) | Espaço: O(n)
pub fn eh_palindromo(frase: &str) -> bool {
    let mut deque: VecDeque<char> = frase
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    while deque.len() > 1 {
        if deque.pop_front() != deque.pop_back() {
            return false;
        }
    }
    true
}

/// 15. Janela Deslizante Máxima (Solução Monotônica O(n))
/// Tempo: O(n) | Espaço: O(k)
pub fn max_janela_deslizante(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let mut deque: VecDeque<usize> = VecDeque::new();
    let mut resultado = Vec::new();
    for i in 0..nums.len() {
        if !deque.is_empty() && deque.front().copied().unwrap() <= i.saturating_sub(k) {
            deque.pop_front();
        }
        while !deque.is_empty() && nums[*deque.back().unwrap()] <= nums[i] {
            deque.pop_back();
        }
        deque.push_back(i);
        if i >= k - 1 {
            resultado.push(nums[*deque.front().unwrap()]);
        }
    }
    resultado
}

/// 19. Fila com iteração controlada (Processar em lotes)
/// Tempo: O(n) | Espaço: O(tamanho_lote)
pub fn processar_em_lotes(fila: &mut VecDeque<i32>, tamanho_lote: usize) {
    while !fila.is_empty() {
        let mut lote = Vec::new();
        for _ in 0..tamanho_lote {
            if let Some(elemento) = fila.pop_front() {
                lote.push(elemento);
            }
        }
        println!("  [Lote Processado]: {:?}", lote);
    }
}

/// 20. Mini-projeto Round Robin
/// Tempo: O(Total_Trabalho / Quantum) | Espaço: O(n)
pub fn simulador_round_robin(mut processos: VecDeque<Processo>, quantum: u32) {
    let mut tempo_total = 0;
    while let Some(mut p) = processos.pop_front() {
        if p.tempo_restante <= quantum {
            tempo_total += p.tempo_restante;
            println!("  [Round Robin] Processo {} finalizado no tempo: {}", p.id, tempo_total);
        } else {
            tempo_total += quantum;
            p.tempo_restante -= quantum;
            processos.push_back(p);
        }
    }
}

// =========================================================================
// ORQUESTRAÇÃO DE TESTE SEGURO (NÃO QUEBRA A MAIN)
// =========================================================================

/// Executa e demonstra todas as soluções da Aula 06 de forma isolada.
pub fn executar_tudo() {
    println!("\n=== [MÓDULO AULA 06] INICIANDO TESTES DOS TADS LINEARES ===");

    // G1-Ex1
    let v = vec![1, 2, 3, 4, 5];
    println!("Ex 1 (Inversão): original {:?} -> invertido {:?}", v, inverter_vetor(v.clone()));

    // G1-Ex2
    let letras = vec!['a', 'b', 'a', 'c', 'b', 'a'];
    println!("Ex 2 (Contador): {:?}", contar_ocorrencias(letras));

    // G1-Ex3
    println!("Ex 3 (Remover Pares): {:?}", remover_pares(vec![1, 2, 3, 4, 5, 6]));

    // G1-Ex4
    println!("Ex 4 (Mescla Ordenada): {:?}", mescla_ordenada(&[1, 3, 5], &[2, 4, 6]));

    // G2-Ex5
    println!("Ex 5 (RPN '3 4 + 2 *'): {}", avaliar_rpn("3 4 + 2 *"));

    // G2-Ex6
    let mut nav = Navegador::new("google.com");
    nav.visitar("github.com");
    nav.voltar();
    println!("Ex 6 (Navegador - Voltou para): {}", nav.atual);

    // G2-Ex7
    let mut ed = EditorTexto::new();
    ed.digitar("Olá ");
    ed.digitar("Mundo");
    ed.desfazer();
    println!("Ex 7 (Editor - Após desfazer): '{}'", ed.texto);

    // G2-Ex8
    println!("Ex 8 (Delimitadores '[{()}]'): {}", delimitadores_balanceados("[{()}]"));

    // G2-Ex9
    let mut s_min = StackMin::new();
    s_min.push(5);
    s_min.push(3);
    s_min.push(7);
    println!("Ex 9 (Mínimo da Pilha): {:?}", s_min.min());

    // G3-Ex10
    let clientes = vec![Cliente { id: 1, tempo_chegada: 0 }, Cliente { id: 2, tempo_chegada: 2 }];
    println!("Ex 10 (Espera do Banco): {:.2} min", calcular_espera_banco(clientes, 3));

    // G3-Ex11
    let mut fila_imp = VecDeque::new();
    fila_imp.push_back(TrabalhoImpressao { nome: "Relatorio.pdf".to_string(), paginas: 12 });
    print!("Ex 11: ");
    simular_impressora(fila_imp);

    // G3-Ex12
    let mut circular = FilaCircular::new(2);
    circular.enqueue("Msg 1".to_string());
    circular.enqueue("Msg 2".to_string());
    circular.enqueue("Msg 3 (Sobrescreve)".to_string());
    println!("Ex 12 (Fila Circular Dequeue): {:?}", circular.dequeue());

    // G3-Ex13
    let mut f_prioridade = FilaPrioridadeManual::new();
    f_prioridade.enqueue(ItemPrioridade { dado: "Baixa".to_string(), prioridade: 1 });
    f_prioridade.enqueue(ItemPrioridade { dado: "Alta".to_string(), prioridade: 10 });
    println!("Ex 13 (Maior Prioridade Retirada): {:?}", f_prioridade.dequeue().map(|i| i.dado));

    // G4-Ex14
    println!("Ex 14 (Palíndromo 'A man a plan a canal Panama'): {}", eh_palindromo("A man a plan a canal Panama"));

    // G4-Ex15
    println!("Ex 15 (Janela Deslizante Máxima): {:?}", max_janela_deslizante(vec![1, 3, -1, -3, 5, 3, 6, 7], 3));

    // G4-Ex16
    let mut t_fila = FilaTarefas::new();
    t_fila.adicionar_normal(Tarefa { nome: "Tarefa Normal".to_string() });
    t_fila.adicionar_urgente(Tarefa { nome: "Tarefa Urgente".to_string() });
    println!("Ex 16 (Próxima tarefa a processar): {:?}", t_fila.processar_proxima().map(|t| t.nome));

    // G5-Ex19
    let mut fila_lotes = VecDeque::from(vec![10, 20, 30, 40, 50]);
    println!("Ex 19 (Processamento em Lotes de tamanho 2):");
    processar_em_lotes(&mut fila_lotes, 2);

    // G5-Ex20
    let mut procs = VecDeque::new();
    procs.push_back(Processo { id: 1, tempo_restante: 10 });
    procs.push_back(Processo { id: 2, tempo_restante: 4 });
    println!("Ex 20 (Simulação Round Robin Quantum = 3):");
    simulador_round_robin(procs, 3);

    println!("=== FIM DOS TESTES DA AULA 06 ===\n");
}
