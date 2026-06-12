// ==========================================
// Grupo 4 – Deque (Double-Ended Queue)
// Deque é novo pra mim, mas VecDeque do Rust ajuda muito
// ==========================================
use std::collections::VecDeque;

// Exercício 14 – Verificar palíndromo com Deque
// Tiro espaços, passo pra minúsculas, comparo as pontas do deque
// Complexidade: O(n)
pub fn eh_palindromo(texto: &str) -> bool {
    let mut deque: VecDeque<char> = texto
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

// Exercício 15 – Janela deslizante (máximo de cada janela)
// Esse foi punk. O Deque guarda ÍNDICES, não valores.
// Mantenho os índices em ordem decrescente do valor correspondente.
// Quando a janela desliza, removo índices que saíram e índices menores que o novo.
// Complexidade: O(n) – cada índice entra e sai do deque no máximo uma vez
pub fn maximo_janelas(vetor: &[i32], k: usize) -> Vec<i32> {
    let mut deque: VecDeque<usize> = VecDeque::new();
    let mut resultado = Vec::new();
    
    for i in 0..vetor.len() {
        // Remove índices que saíram da janela atual
        while deque.front().map_or(false, |&f| f + k <= i) {
            deque.pop_front();
        }
        
        // Remove índices cujo valor é menor ou igual ao atual
        // (não serão úteis nas próximas janelas)
        while deque.back().map_or(false, |&b| vetor[b] <= vetor[i]) {
            deque.pop_back();
        }
        
        deque.push_back(i);
        
        // Janela completa?
        if i + 1 >= k {
            resultado.push(vetor[*deque.front().unwrap()]);
        }
    }
    
    resultado
}

// Exercício 16 – Fila de tarefas com prioridade de frente
// Tarefas urgentes → push_front, tarefas normais → push_back
// Processamento sempre pega da frente (pop_front)
// Complexidade: O(1) amortizado por operação
pub struct FilaTarefas {
    tarefas: VecDeque<String>,
}

impl FilaTarefas {
    pub fn nova() -> Self {
        FilaTarefas {
            tarefas: VecDeque::new(),
        }
    }
    
    pub fn adicionar_urgente(&mut self, tarefa: &str) {
        self.tarefas.push_front(tarefa.to_string());
    }
    
    pub fn adicionar_normal(&mut self, tarefa: &str) {
        self.tarefas.push_back(tarefa.to_string());
    }
    
    pub fn processar(&mut self) -> Option<String> {
        self.tarefas.pop_front()
    }
    
    pub fn tamanho(&self) -> usize {
        self.tarefas.len()
    }
}
