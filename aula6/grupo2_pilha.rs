// ==========================================
// Grupo 2 – Pilha (Stack)
// Pilha é de boa porque Vec já faz push/pop naturalmente
// ==========================================

// Exercício 5 – Calculadora RPN (notação polonesa reversa)
// Uso Vec<f64> como pilha. Número → empilha, operador → desempilha dois, calcula, empilha resultado
// Complexidade: O(n) – cada token é processado uma vez
pub fn calculadora_rpn(expressao: &str) -> f64 {
    let mut pilha: Vec<f64> = Vec::new();
    
    for token in expressao.split_whitespace() {
        match token {
            "+" => {
                let b = pilha.pop().unwrap();
                let a = pilha.pop().unwrap();
                pilha.push(a + b);
            }
            "-" => {
                let b = pilha.pop().unwrap();
                let a = pilha.pop().unwrap();
                pilha.push(a - b);
            }
            "*" => {
                let b = pilha.pop().unwrap();
                let a = pilha.pop().unwrap();
                pilha.push(a * b);
            }
            "/" => {
                let b = pilha.pop().unwrap();
                let a = pilha.pop().unwrap();
                pilha.push(a / b);
            }
            numero => {
                let n: f64 = numero.parse().unwrap();
                pilha.push(n);
            }
        }
    }
    
    pilha.pop().unwrap()
}

// Exercício 6 – Histórico de navegação (Voltar/Avançar)
// Duas pilhas: uma pra trás e outra pra frente. Quando visita página nova, limpa a de frente.
// Complexidade: O(1) por operação
pub struct Navegador {
    historico_voltar: Vec<String>,
    historico_avancar: Vec<String>,
    pagina_atual: String,
}

impl Navegador {
    pub fn novo() -> Self {
        Navegador {
            historico_voltar: Vec::new(),
            historico_avancar: Vec::new(),
            pagina_atual: String::new(),
        }
    }
    
    pub fn visitar(&mut self, url: &str) {
        if !self.pagina_atual.is_empty() {
            self.historico_voltar.push(self.pagina_atual.clone());
        }
        self.pagina_atual = url.to_string();
        self.historico_avancar.clear(); // limpando histórico de avançar
    }
    
    pub fn voltar(&mut self) -> bool {
        if let Some(pagina_anterior) = self.historico_voltar.pop() {
            self.historico_avancar.push(self.pagina_atual.clone());
            self.pagina_atual = pagina_anterior;
            return true;
        }
        false
    }
    
    pub fn avancar(&mut self) -> bool {
        if let Some(proxima_pagina) = self.historico_avancar.pop() {
            self.historico_voltar.push(self.pagina_atual.clone());
            self.pagina_atual = proxima_pagina;
            return true;
        }
        false
    }
    
    pub fn atual(&self) -> &str {
        &self.pagina_atual
    }
}

// Exercício 7 – Desfazer/Refazer (editor de texto)
// Mesma lógica do navegador: duas pilhas, uma de ações desfeitas e outra de refeitas
// Complexidade: O(tamanho do texto) por causa do clone (dá pra otimizar depois)
pub struct Editor {
    texto: String,
    desfeitos: Vec<String>,
    refeitos: Vec<String>,
}

impl Editor {
    pub fn novo() -> Self {
        Editor {
            texto: String::new(),
            desfeitos: Vec::new(),
            refeitos: Vec::new(),
        }
    }
    
    pub fn digitar(&mut self, texto: &str) {
        self.desfeitos.push(self.texto.clone());
        self.refeitos.clear();
        self.texto.push_str(texto);
    }
    
    pub fn desfazer(&mut self) -> bool {
        if let Some(anterior) = self.desfeitos.pop() {
            self.refeitos.push(self.texto.clone());
            self.texto = anterior;
            return true;
        }
        false
    }
    
    pub fn refazer(&mut self) -> bool {
        if let Some(proximo) = self.refeitos.pop() {
            self.desfeitos.push(self.texto.clone());
            self.texto = proximo;
            return true;
        }
        false
    }
    
    pub fn texto_atual(&self) -> &str {
        &self.texto
    }
}

// Exercício 8 – Verificar delimitadores balanceados
// Empilho quando abre, comparo topo quando fecha. Se sobrar algo, tá errado.
// Complexidade: O(n)
pub fn balanceado(expressao: &str) -> bool {
    let mut pilha = Vec::new();
    
    for c in expressao.chars() {
        match c {
            '(' | '[' | '{' => pilha.push(c),
            ')' => {
                if pilha.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if pilha.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if pilha.pop() != Some('{') {
                    return false;
                }
            }
            _ => {} // ignora outros caracteres
        }
    }
    
    pilha.is_empty()
}

// Exercício 9 – Pilha que retorna mínimo em O(1)
// Usei uma pilha auxiliar que guarda o menor valor até aquele momento
// Complexidade: O(1) em push, pop e min()
pub struct PilhaMin {
    pilha: Vec<i32>,
    minimos: Vec<i32>,
}

impl PilhaMin {
    pub fn nova() -> Self {
        PilhaMin {
            pilha: Vec::new(),
            minimos: Vec::new(),
        }
    }
    
    pub fn push(&mut self, valor: i32) {
        self.pilha.push(valor);
        let novo_min = match self.minimos.last() {
            Some(&atual) if atual < valor => atual,
            _ => valor,
        };
        self.minimos.push(novo_min);
    }
    
    pub fn pop(&mut self) -> Option<i32> {
        self.minimos.pop();
        self.pilha.pop()
    }
    
    pub fn minimo(&self) -> Option<i32> {
        self.minimos.last().copied()
    }
}
