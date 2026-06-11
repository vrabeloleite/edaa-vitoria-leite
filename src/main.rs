use std::collections::{HashMap, VecDeque};

// =========================================================================
// GRUPO 1: VEC E OPERAÇÕES BÁSICAS
// =========================================================================

// Exercício 1: Inversão utilizando apenas manipulação de pontas (push/pop)
// Custo de Tempo: O(n) | Custo de Espaço: O(n)
pub fn inverter_vetor(mut vec: Vec<i32>) -> Vec<i32> {
    let mut aux = Vec::new();
    while let Some(elemento) = vec.pop() {
        aux.push(elemento);
    }
    aux
}

// Exercício 2: Contador de ocorrências por referência estável
// Custo de Tempo: O(n) | Custo de Espaço: O(k)
pub fn contar_ocorrencias(vec: Vec<char>) -> HashMap<char, usize> {
    let mut contador = HashMap::new();
    for x in &vec {
        *contador.entry(*x).or_insert(0) += 1;
    }
    contador
}

// Exercício 3: Remoção de pares via filtragem linear
// Custo de Tempo: O(n) | Custo de Espaço: O(n)
pub fn remover_pares(vec: Vec<i32>) -> Vec<i32> {
    let mut impares = Vec::new();
    for num in vec {
        if num % 2 != 0 {
            impares.push(num);
        }
    }
    impares
}

// Exercício 4: Interpolação / Mescla de vetores ordenados (Two-Pointers)
// Custo de Tempo: O(n + m) | Custo de Espaço: O(n + m)
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

// =========================================================================
// GRUPO 2: PILHA (STACK)
// =========================================================================

// Exercício 5: Avaliação de Expressões RPN Corrigida (Evitando Double Borrow)
// Custo de Tempo: O(n) | Custo de Espaço: O(n)
pub fn avaliar_rpn(expressao: &str) -> f64 {
    let mut pilha: Vec<f64> = Vec::new();
    for token in expressao.split_whitespace() {
        match token {
            "+" | "-" | "*" | "/" => {
                let b = pilha.pop();
                let a = pilha.pop();
                if let (Some(b_val), Some(a_val)) = (b, a) {
                    let res = match token {
                        "+" => a_val + b_val,
                        "-" => a_val - b_val,
                        "*" => a_val * b_val,
                        "/" => a_val / b_val,
                        _ => 0.0,
                    };
                    pilha.push(res);
                }
            }
            num => {
                if let Ok(val) = num.parse::<f64>() {
                    pilha.push(val);
                }
            }
        }
    }
    pilha.pop().unwrap_or(0.0)
}

// Exercício 6: Controle de Fluxo de Navegação (Back/Forward)
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

// Exercício 7: Mecanismo Undo/Redo para Editor Minimalista
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

// Exercício 8: Validação de Escopo e Sintaxe de Delimitadores
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

// Exercício 9: Estrutura StackMin com tracking de mínimo em tempo real
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

// =========================================================================
// GRUPO 3: FILA (QUEUE)
// =========================================================================

pub struct Cliente {
    pub id: usize,
    pub tempo_chegada: u32,
}

// Exercício 10: Modelo de Simulação de Atendimento Bancário (FIFO)
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

pub struct TrabalhoImpressao {
    pub nome: String,
    pub paginas: u32,
}

// Exercício 11: Simulação de Spooler de Impressão Compartilhada
pub fn simular_impressora(mut fila: VecDeque<TrabalhoImpressao>) {
    while let Some(t) = fila.pop_front() {
        println!("  [Spooler] Impresso: {} ({} páginas)", t.nome, t.paginas);
    }
}

// Exercício 12: Buffer de Mensagens Estático (Fila Circular com Sobrescrita)
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

pub struct ItemPrioridade {
    pub dado: String,
    pub prioridade: i32,
}

// Exercício 13: Fila de Prioridade Manual via Varredura Linear
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

// =========================================================================
// GRUPO 4: DEQUE (DOUBLE-ENDED QUEUE)
// =========================================================================

// Exercício 14: Verificador de Palíndromos via Deque
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

// Exercício 15: Janela Deslizante Máxima (Fila Monotônica Decrescente)
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

#[derive(Debug)]
pub struct Tarefa {
    pub nome: String,
}

// Exercício 16: Agendador de Tarefas Híbrido com Intervenção Frontal
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

// =========================================================================
// GRUPO 5: FILA COM ITERAÇÃO CONTROLADA E ROUND ROBIN
// =========================================================================

// Exercício 19: Processamento Batch de Estruturas FIFO
pub fn processar_em_lotes(fila: &mut VecDeque<i32>, tamanho_lote: usize) {
    while !fila.is_empty() {
        let mut lote = Vec::new();
        for _ in 0..tamanho_lote {
            if let Some(elemento) = fila.pop_front() {
                lote.push(elemento);
            }
        }
        println!("  [Batch] Lote Consumido: {:?}", lote);
    }
}

#[derive(Clone, Debug)]
pub struct Processo {
    pub id: usize,
    pub tempo_restante: u32,
}

// Exercício 20: Algoritmo de Escalonamento de CPU Round-Robin
pub fn simulador_round_robin(mut processos: VecDeque<Processo>, quantum: u32) {
    let mut tempo_total = 0;
    while let Some(mut p) = processos.pop_front() {
        if p.tempo_restante <= quantum {
            tempo_total += p.tempo_restante;
            println!("  [CPU] Processo ID {} terminado no ciclo: {}", p.id, tempo_total);
        } else {
            tempo_total += quantum;
            p.tempo_restante -= quantum;
            processos.push_back(p);
        }
    }
}

// =========================================================================
// RUNNER INTERNO PARA CHAMADA MANUAL VIA MAIN
// =========================================================================
pub fn testar_atividades_aula06() {
    println!("\n>>> RUNNER: EXECUTANDO AULA 06 MOCADO <<<");
    assert_eq!(inverter_vetor(vec![1, 2, 3]), vec![3, 2, 1]);
    assert_eq!(avaliar_rpn("3 4 + 2 *"), 14.0);
    println!(">>> TODOS OS PRINTS EXECUTADOS SEM ERROS NO TERMINAL <<<\n");
}

// =========================================================================
// 🚀 SUÍTE DE TESTES UNITÁRIOS NATIVOS DO RUST (PARA CARGO TEST / GITHUB ACTIONS)
// =========================================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grupo1_vecs() {
        assert_eq!(inverter_vetor(vec![1, 2, 3, 4, 5]), vec![5, 4, 3, 2, 1]);
        
        let ocorrencias = contar_ocorrencias(vec!['a', 'b', 'a']);
        assert_eq!(*ocorrencias.get(&'a').unwrap(), 2);
        
        assert_eq!(remover_pares(vec![1, 2, 3, 4, 6, 7]), vec![1, 3, 7]);
        assert_eq!(mescla_ordenada(&[1, 3, 5], &[2, 4]), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_grupo2_pilhas() {
        assert_eq!(avaliar_rpn("3 4 + 2 *"), 14.0);
        
        let mut nav = Navegador::new("home.com");
        nav.visitar("aula.com");
        nav.voltar();
        assert_eq!(nav.atual, "home.com");
        
        assert!(delimitadores_balanceados("[{()}]"));
        assert!(!delimitadores_balanceados("[{(})]"));
        
        let mut s_min = StackMin::new();
        s_min.push(5);
        s_min.push(2);
        s_min.push(9);
        assert_eq!(s_min.min(), Some(2));
    }

    #[test]
    fn test_grupo3_filas() {
        let clientes = vec![Cliente { id: 1, tempo_chegada: 0 }, Cliente { id: 2, tempo_chegada: 1 }];
        assert!(calcular_espera_banco(clientes, 2) >= 0.0);
        
        let mut circular = FilaCircular::new(2);
        circular.enqueue("A".to_string());
        circular.enqueue("B".to_string());
        circular.enqueue("C".to_string()); // Sobrescreve 'A'
        assert_eq!(circular.dequeue(), Some("B".to_string()));
    }

    #[test]
    fn test_grupo4_deques() {
        assert!(eh_palindromo("A man a plan a canal Panama"));
        assert_eq!(max_janela_deslizante(vec![1, 3, -1, -3, 5, 3, 6, 7], 3), vec![3, 3, 5, 5, 6, 7]);
    }
}
