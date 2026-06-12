// ==========================================
// Grupo 5 – Códigos que faltavam
// Exercícios 17, 19 e 20 (os que precisam de código)
// ==========================================
use std::collections::VecDeque;
use std::time::Instant;

// =============================================
// Exercício 17 – Comparação de desempenho
// Testei três tipos de fila: Vec ingênua, VecDeque e Fila Circular
// =============================================

/// Fila ingênua com Vec (ruim: pop_front é O(n))
struct FilaIngenua {
    dados: Vec<i32>,
}
impl FilaIngenua {
    fn nova() -> Self { FilaIngenua { dados: Vec::new() } }
    fn enfileirar(&mut self, v: i32) { self.dados.push(v); }
    fn desenfileirar(&mut self) -> Option<i32> {
        if self.dados.is_empty() { None }
        else { Some(self.dados.remove(0)) } // O(n) - shift nos elementos
    }
}

/// Fila circular simples com array fixo
struct FilaCircular {
    dados: Vec<Option<i32>>,
    head: usize,
    tail: usize,
    tamanho: usize,
    capacidade: usize,
}
impl FilaCircular {
    fn nova(cap: usize) -> Self {
        FilaCircular {
            dados: (0..cap).map(|_| None).collect(),
            head: 0, tail: 0, tamanho: 0, capacidade: cap,
        }
    }
    fn enfileirar(&mut self, v: i32) {
        if self.tamanho == self.capacidade { return; } // cheia
        self.dados[self.tail] = Some(v);
        self.tail = (self.tail + 1) % self.capacidade;
        self.tamanho += 1;
    }
    fn desenfileirar(&mut self) -> Option<i32> {
        if self.tamanho == 0 { return None; }
        let v = self.dados[self.head].take();
        self.head = (self.head + 1) % self.capacidade;
        self.tamanho -= 1;
        v
    }
}

pub fn comparar_filas() {
    const N: usize = 10_000;

    // Vec ingênua
    let inicio = Instant::now();
    let mut f1 = FilaIngenua::nova();
    for i in 0..N { f1.enfileirar(i as i32); }
    for _ in 0..N { f1.desenfileirar(); }
    let t1 = inicio.elapsed();

    // VecDeque
    let inicio = Instant::now();
    let mut f2 = VecDeque::new();
    for i in 0..N { f2.push_back(i as i32); }
    for _ in 0..N { f2.pop_front(); }
    let t2 = inicio.elapsed();

    // Fila circular
    let inicio = Instant::now();
    let mut f3 = FilaCircular::nova(N + 1);
    for i in 0..N { f3.enfileirar(i as i32); }
    for _ in 0..N { f3.desenfileirar(); }
    let t3 = inicio.elapsed();

    println!("=== Comparação de Filas ({} operações) ===", N);
    println!("Vec ingênua:  {:?}", t1);
    println!("VecDeque:     {:?}", t2);
    println!("Fila circular: {:?}", t3);
}

// =============================================
// Exercício 19 – Processamento em lotes
// Uso drain() do VecDeque pra pegar pedaços da frente
// Complexidade: O(n) total
// =============================================

pub fn processar_em_lotes(fila: &mut VecDeque<i32>, tamanho_lote: usize) {
    let mut lote_num = 1;
    while !fila.is_empty() {
        let qtd = std::cmp::min(tamanho_lote, fila.len());
        let lote: Vec<_> = fila.drain(..qtd).collect();
        println!("Lote {}: {:?}", lote_num, lote);
        lote_num += 1;
    }
}

// =============================================
// Exercício 20 – Round Robin
// Cada processo tem id e tempo total de execução.
// Uso VecDeque como fila circular.
// Quantum = fatia de tempo que cada processo ganha por rodada.
// =============================================

#[derive(Debug, Clone)]
pub struct Processo {
    pub id: usize,
    pub tempo_total: u32,
}

pub fn round_robin(processos: Vec<Processo>, quantum: u32) -> Vec<(usize, u32)> {
    // Fila com tupla (id, tempo_restante)
    let mut fila: VecDeque<(usize, u32)> = processos
        .iter()
        .map(|p| (p.id, p.tempo_total))
        .collect();
    
    let mut tempo_atual: u32 = 0;
    let mut conclusoes: Vec<(usize, u32)> = Vec::new();
    
    println!("\n=== Round Robin (quantum = {}) ===", quantum);
    
    while let Some((id, restante)) = fila.pop_front() {
        if restante <= quantum {
            // Processo termina agora
            tempo_atual += restante;
            conclusoes.push((id, tempo_atual));
            println!("Processo {} terminou no tempo {}", id, tempo_atual);
        } else {
            // Processo não terminou, volta pro final
            tempo_atual += quantum;
            let novo_restante = restante - quantum;
            println!("Processo {} executou {} (falta {})", id, quantum, novo_restante);
            fila.push_back((id, novo_restante));
        }
    }
    
    conclusoes
}

// Teste simples (não é #[test] pra poder rodar com println)
pub fn testar_round_robin() {
    let processos = vec![
        Processo { id: 1, tempo_total: 10 },
        Processo { id: 2, tempo_total: 5 },
        Processo { id: 3, tempo_total: 8 },
    ];
    let resultado = round_robin(processos, 3);
    println!("Conclusões: {:?}", resultado);
}
