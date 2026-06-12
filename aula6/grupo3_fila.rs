// ==========================================
// Grupo 3 – Fila (Queue)
// Fila é FIFO, usei VecDeque porque é eficiente nas duas pontas
// ==========================================
use std::collections::VecDeque;

// Exercício 10 – Simulador de fila de banco
// Versão simplificada, depois posso adicionar tempos aleatórios com a crate rand
// Complexidade: O(n) onde n é número de clientes
pub fn simular_banco() {
    println!("--- Simulador de Banco ---");
    let mut fila: VecDeque<u32> = VecDeque::new();
    
    // Clientes chegando
    for i in 1..=5 {
        fila.push_back(i);
        println!("Cliente {} chegou na fila", i);
    }
    
    // Atendendo
    while let Some(cliente) = fila.pop_front() {
        println!("Atendendo cliente {}...", cliente);
    }
    println!("Fila vazia!\n");
}

// Exercício 11 – Impressora compartilhada
// Cada trabalho tem nome e número de páginas. Processo na ordem de chegada.
// Complexidade: O(n) total
pub struct Trabalho {
    pub nome: String,
    pub paginas: u32,
}

pub fn simular_impressora(trabalhos: Vec<Trabalho>) {
    println!("--- Impressora Compartilhada ---");
    let mut fila: VecDeque<Trabalho> = VecDeque::from(trabalhos);
    
    while let Some(trabalho) = fila.pop_front() {
        println!("Imprimindo '{}' ({} páginas)...", trabalho.nome, trabalho.paginas);
        println!("✓ Concluído: {}\n", trabalho.nome);
    }
}

// Exercício 12 – Buffer circular com overwrite
// Quando enche, descarta a mensagem mais antiga (pop_front) e insere a nova
// Complexidade: O(1) amortizado por operação
pub struct BufferCircular {
    dados: VecDeque<String>,
    capacidade: usize,
}

impl BufferCircular {
    pub fn novo(capacidade: usize) -> Self {
        BufferCircular {
            dados: VecDeque::with_capacity(capacidade),
            capacidade,
        }
    }
    
    pub fn push(&mut self, mensagem: &str) {
        if self.dados.len() == self.capacidade {
            self.dados.pop_front(); // descarta a mais antiga
        }
        self.dados.push_back(mensagem.to_string());
    }
    
    pub fn ver_todas(&self) -> Vec<&String> {
        self.dados.iter().collect()
    }
    
    pub fn esta_cheio(&self) -> bool {
        self.dados.len() == self.capacidade
    }
}

// Exercício 13 – Fila de prioridade (busca linear)
// Guardo tupla (prioridade, contador, valor). O contador garante FIFO pra mesma prioridade.
// Remoção é O(n) porque busco o menor linearmente (depois aprendemos heap pra melhorar isso)
// Complexidade: inserção O(1), remoção O(n)
pub struct FilaPrioridade {
    itens: Vec<(i32, u32, String)>, // (prioridade, ordem_chegada, valor)
    contador: u32,
}

impl FilaPrioridade {
    pub fn nova() -> Self {
        FilaPrioridade {
            itens: Vec::new(),
            contador: 0,
        }
    }
    
    pub fn inserir(&mut self, prioridade: i32, valor: &str) {
        self.itens.push((prioridade, self.contador, valor.to_string()));
        self.contador += 1;
    }
    
    pub fn remover(&mut self) -> Option<String> {
        if self.itens.is_empty() {
            return None;
        }
        
        // Acha o índice do elemento de maior prioridade (menor número)
        let mut idx_escolhido = 0;
        for i in 1..self.itens.len() {
            // Prioridade menor = mais importante
            if self.itens[i].0 < self.itens[idx_escolhido].0 {
                idx_escolhido = i;
            } else if self.itens[i].0 == self.itens[idx_escolhido].0 {
                // Mesma prioridade: pega quem chegou primeiro (menor contador)
                if self.itens[i].1 < self.itens[idx_escolhido].1 {
                    idx_escolhido = i;
                }
            }
        }
        
        let (_, _, valor) = self.itens.remove(idx_escolhido);
        Some(valor)
    }
    
    pub fn esta_vazia(&self) -> bool {
        self.itens.is_empty()
    }
}
