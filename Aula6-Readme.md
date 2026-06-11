## Exercícios de Análise Teórica — Aula 06 (TADs Lineares)

### Exercício 17: Relatório de Desempenho (Cenário de 10.000 Elementos)
* **Fila Baseada em Vec (Ingênua):** Apresentou o pior desempenho prático com complexidade **O(n)** nas operações de remoção frontal (`pop_front` emulado). Isso acontece porque remover o índice zero obriga o sistema operacional a deslocar em bloco todas as posições seguintes de memória.
* **VecDeque (Standard Library):** Desempenho linear estável em **O(1)**. O array circular lógico desloca apenas ponteiros aritméticos de início e término, eliminando movimentação física de bytes.
* **FilaCircular (Customizada):** Teve desempenho idêntico ao `VecDeque` com custo **O(1)**, demonstrando que buffers fixos pré-alocados isolam a aplicação de sofrer com alocações dinâmicas custosas em tempo de execução.

### Exercício 18: Critério de Escolha de Estruturas de Dados
* **(a) Comando "Ctrl+Z":** **Pilha (Stack)**. Estrutura regida estritamente por LIFO (Last-In, First-Out). O último estado do documento precisa obrigatoriamente ser o primeiro a ser desempilhado e revertido.
* **(b) Despacho de Pedidos:** **Fila (Queue)**. Uso obrigatório de comportamento FIFO (First-In, First-Out) para garantir a integridade e cronologia do atendimento por prioridade de chegada.
* **(c) Validador de Tags HTML:** **Pilha (Stack)**. Essencial para analisar o aninhamento e o escopo da marcação. Tags de abertura entram na pilha, e tags de fechamento validam a integridade comparando-se diretamente com o elemento presente no topo atual.
* **(d) Busca em Largura (BFS):** **Fila (Queue)**. Garante que os vértices de uma subárvore ou diretório sejam exauridos em nível de profundidade atual de forma ordenada antes que o algoritmo explore novos níveis hierárquicos abaixo.
* **(e) Análise de Palíndromo:** **Deque (Double-Ended Queue)**. Permite a checagem paralela e simétrica comparando simultaneamente os caracteres posicionados nas duas pontas opostas da string com custo de tempo O(1).
