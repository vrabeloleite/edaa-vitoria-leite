# Grupo 5 – Reflexão e Análise

## Exercício 17 – Comparação de desempenho

Implementei três tipos de fila e medi com `std::time::Instant`:

- **Vec ingênua:** `push` no final + `remove(0)` → O(n) na remoção
- **VecDeque:** `push_back` + `pop_front` → O(1) amortizado
- **Fila circular:** array fixo + índices → O(1)

Resultados (10.000 operações, meu note i5 8GB):

| Implementação | Tempo aprox. | 
|---------------|--------------|
| Vec ingênua | ~15ms |
| VecDeque | ~1ms |
| Fila circular | ~0.8ms |

**Conclusão:** Vec puro como fila é terrível. VecDeque já resolve bem. 
Fila circular é marginalmente melhor, mas mais chata de implementar.

---

## Exercício 18 – Quando usar qual TAD?

**(a) Botão Ctrl+Z de um editor**  
→ **Pilha** – a última ação feita é a primeira a ser desfeita (LIFO).

**(b) Processar pedidos de um restaurante**  
→ **Fila** – primeiro pedido a entrar é o primeiro a ser preparado (FIFO).

**(c) Verificar tags HTML balanceadas**  
→ **Pilha** – ao abrir `<div>`, empilha; ao fechar `</div>`, verifica se o topo é `<div>`.

**(d) Navegar em arquivos de um diretório em largura (BFS)**  
→ **Fila** – visita o diretório raiz, depois todos os filhos, depois netos...

**(e) Verificar se uma sequência de palavras é palíndromo**  
→ **Deque** – compara as palavras das pontas para o centro.

---

## Exercício 19 – Processamento em lotes

Usei `VecDeque` com o método `drain()` que já pega um pedaço da frente.

```rust
fn processar_em_lotes(fila: &mut VecDeque<i32>, tamanho_lote: usize) {
    while !fila.is_empty() {
        let quantidade = std::cmp::min(tamanho_lote, fila.len());
        let lote: Vec<_> = fila.drain(..quantidade).collect();
        println!("Processando lote: {:?}", lote);
    }
}
