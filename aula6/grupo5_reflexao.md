# Grupo 5 – Reflexão e Análise

## Exercício 17 – Comparação de desempenho

Implementei três tipos de fila e medi com `std::time::Instant`:

- **Vec ingênua:** `push` no final + `remove(0)` → O(n) na remoção (desloca todo mundo)
- **VecDeque:** `push_back` + `pop_front` → O(1) amortizado
- **Fila circular:** array fixo com índices `head` e `tail` → O(1) de verdade

**Código:** está em `grupo5_extras.rs` na função `comparar_filas()`

Resultados no meu note (i5 8GB, 10.000 operações):

| Implementação | Tempo aprox. | Por quê? |
|---------------|--------------|----------|
| Vec ingênua | ~15ms | `remove(0)` desloca os N elementos toda vez |
| VecDeque | ~1ms | anel interno, sem deslocamento |
| Fila circular | ~0.8ms | acesso direto, mas diferença pequena pro VecDeque |

**Conclusão:** Nunca usar Vec puro como fila! VecDeque do Rust já resolve 99% dos casos.

---

## Exercício 18 – Quando usar qual TAD?

**(a) Botão Ctrl+Z de um editor**  
→ **Pilha** – a última ação feita é a primeira a ser desfeita (LIFO).  
Ex: digitou "a", "b", "c" → Ctrl+Z desfaz "c", depois "b", depois "a".

**(b) Processar pedidos de um restaurante**  
→ **Fila** – primeiro pedido a entrar é o primeiro a ser preparado (FIFO).  
Ex: cliente 1 pediu pizza, cliente 2 pediu hambúrguer → pizza sai primeiro.

**(c) Verificar tags HTML balanceadas**  
→ **Pilha** – ao abrir `<div>`, empilha; ao fechar `</div>`, verifica se o topo é `<div>`.  
Ex: `<html><body></body></html>` → OK. `<html><body></html>` → erro.

**(d) Navegar em arquivos de um diretório em largura (BFS)**  
→ **Fila** – visita o diretório raiz, depois todos os filhos, depois netos, etc.  
Ex: `/home` → `/home/docs`, `/home/photos` → `/home/docs/aula6`...

**(e) Verificar se uma sequência de palavras é palíndromo**  
→ **Deque** – compara as palavras das pontas para o centro.  
Ex: "casa carro barco carro casa" → Deque tira "casa" das duas pontas, ok.

---

## Exercício 19 – Processamento em lotes

Usei `VecDeque` com o método `drain()` que já pega um pedaço da frente de uma vez.

**Código:** está em `grupo5_extras.rs` na função `processar_em_lotes()`

Funcionamento:
- Recebe um `&mut VecDeque<i32>` e um `tamanho_lote`
- Enquanto a fila não estiver vazia:
  - Calcula quantos itens pegar (mínimo entre tamanho_lote e o que sobrou)
  - Usa `drain(..qtd)` pra remover e coletar de uma vez
  - Imprime o lote

Complexidade: O(n) total porque cada elemento é removido exatamente uma vez.

---

## Exercício 20 – Round Robin (mini-projeto)

Esse foi o mais legal, mas deu um trabalhinho.

**Código:** está em `grupo5_extras.rs` na função `round_robin()`

### Como funciona:
1. Cada processo tem `id` e `tempo_total` de execução
2. Fila circular com `VecDeque<(id, tempo_restante)>`
3. Enquanto tem processo na fila:
   - Tira o primeiro da fila (`pop_front`)
   - Se `tempo_restante <= quantum` → processo termina, registra conclusão
   - Senão → executa o quantum, subtrai do restante, põe de volta no final (`push_back`)
4. No final, retorna lista de `(id, tempo_conclusao)`

### Exemplo:
- Processo 1: 10ms, Processo 2: 5ms, Processo 3: 8ms
- Quantum = 3ms
- Ordem: P1(3) → P2(3) → P3(3) → P1(3) → P2(2, acabou!) → P3(3) → P1(3) → P3(2, acabou!) → P1(1, acabou!)

**Primeira tentativa deu loop infinito** porque esqueci de tratar quando o restante era menor que o quantum. Arrumei colocando um `if restante <= quantum`.

### Complexidade:
- Cada processo é colocado na fila `ceil(tempo_total / quantum)` vezes
- Total: O(Σ teto(tempo_i / quantum)) operações
