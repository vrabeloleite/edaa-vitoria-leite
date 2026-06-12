# Aula 06 – TADs Lineares (Vec, Pilha, Fila, Deque)

**Data:** 30/03/2026  
**UC:** Estruturas de Dados e Análise de Algoritmos

## O que eu fiz

Implementei os 20 exercícios separados por grupo:

- **Grupo 1 (Vec):** Inversão, contagem, remoção condicional, mescla ordenada
- **Grupo 2 (Pilha):** Calculadora RPN, navegador, editor, balanceamento, pilha com mínimo
- **Grupo 3 (Fila):** Simulador banco, impressora, buffer circular, fila de prioridade
- **Grupo 4 (Deque):** Palíndromo, janela deslizante, fila de tarefas
- **Grupo 5 (Reflexão):** Comparação de desempenho, escolha de TAD, processamento em lotes, round robin

## Dificuldades que tive

- **Exercício 15 (janela deslizante):** Foi o mais difícil. Tive que pesquisar como usar o Deque pra guardar índices em ordem decrescente. Demorei pra entender, mas depois fez sentido.
- **Exercício 12 (buffer circular):** No começo não entendi que o overwrite era pra descartar o mais antigo, não recusar o novo.
- **Exercício 9 (pilha com mínimo):** A ideia da pilha auxiliar é genial, não tinha pensado nisso sozinho.
- **Round robin (ex 20):** Primeira tentativa entrou em loop infinito porque esqueci de tratar quando o tempo restante é menor que o quantum.

## Complexidades (resumo rápido)

| Estrutura | Inserir | Remover | Observação |
|-----------|---------|---------|------------|
| Vec (final) | O(1)* | O(1)* | *amortizado |
| Pilha | O(1) | O(1) | |
| VecDeque | O(1)* | O(1)* | nas duas pontas |
| Fila circular | O(1) | O(1) | |
| Fila prioridade (busca) | O(1) | O(n) | |

---

## Como rodar

```bash
cargo run
cargo test
