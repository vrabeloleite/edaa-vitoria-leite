```rust
// ==========================================
// Grupo 1 – Vec e operações básicas
// Comecei por aqui porque já manjava um pouco de Vec
// ==========================================

// Exercício 1 – Inverter Vec usando só push/pop
// Ideia: tiro do final e jogo num novo vetor, o último vira primeiro
// Complexidade: O(n) – cada elemento é movido uma vez
pub fn inverter(v: &mut Vec<i32>) {
    let mut invertido = Vec::new();
    while let Some(x) = v.pop() {
        invertido.push(x);
    }
    *v = invertido;
}

// Exercício 2 – Contar ocorrências de letras
// Usei HashMap porque é mais direto pra contar coisas
// Complexidade: O(n)
use std::collections::HashMap;
pub fn contar_ocorrencias(v: &Vec<char>) -> HashMap<char, u32> {
    let mut mapa = HashMap::new();
    for &c in v {
        let contador = mapa.entry(c).or_insert(0);
        *contador += 1;
    }
    mapa
}

// Exercício 3 – Remover números pares
// Primeiro tentei com um for e remove(), mas bagunçou os índices
// Aí lembrei que podia filtrar num vetor novo e substituir
// Complexidade: O(n) – filtro percorre uma vez e collect monta outro vetor
pub fn remover_pares(v: &mut Vec<i32>) {
    let novo: Vec<i32> = v.iter().filter(|&&x| x % 2 != 0).cloned().collect();
    *v = novo;
}

// Exercício 4 – Mesclar dois vetores ordenados
// Fiz manual com dois ponteiros, igual o professor mostrou na aula
// Complexidade: O(n + m) – percorro os dois vetores uma vez cada
pub fn mesclar_ordenado(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut resultado = Vec::new();
    let (mut i, mut j) = (0, 0);
    
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            resultado.push(a[i]);
            i += 1;
        } else {
            resultado.push(b[j]);
            j += 1;
        }
    }
    
    // Adiciona as sobras de um dos vetores
    while i < a.len() {
        resultado.push(a[i]);
        i += 1;
    }
    while j < b.len() {
        resultado.push(b[j]);
        j += 1;
    }
    
    resultado
}

// Testes rápidos pra conferir se não fiz besteira
#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn test_inverter() {
        let mut v = vec![1, 2, 3, 4, 5];
        inverter(&mut v);
        assert_eq!(v, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_remover_pares() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        remover_pares(&mut v);
        assert_eq!(v, vec![1, 3, 5]);
    }

    #[test]
    fn test_mesclar() {
        let a = vec![1, 3, 5, 7];
        let b = vec![2, 4, 6];
        assert_eq!(mesclar_ordenado(&a, &b), vec![1, 2, 3, 4, 5, 6, 7]);
    }
}
