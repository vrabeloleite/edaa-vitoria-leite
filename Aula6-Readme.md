## Aula 06: TADs Lineares (Exercícios Teóricos)

### Exercício 17: Comparação de Desempenho (10.000 Elementos) [cite: 35]
* **Vec (Abordagem Ingênua):** Operações como `pop_front` (simuladas por remoções no índice inicial) possuem complexidade de tempo **O(n)**. Isso ocorre devido ao deslocamento físico na memória de todos os elementos subsequentes a cada remoção, tornando a estrutura extremamente ineficiente para grandes volumes de dados.
* **VecDeque (Fila Padrão do Rust):** Utiliza um buffer circular que gerencia ponteiros lógicos de início e fim. Isso garante inserções e remoções estáveis em tempo **O(1)**.
* **FilaCircular (Customizada):** Implementada estaticamente com tamanho fixo, mantendo a performance de tempo estável em **O(1)** sem realocações de memória ou deslocamentos físicos de dados.

### Exercício 18: Quando usar qual TAD? (Justificativas) [cite_start][cite: 37]
* **(a) Botão "Ctrl+Z" de um editor:** **Pilha (Stack)**. [cite_start]Necessita da política LIFO (Last-In, First-Out), de modo que a última alteração efetuada pelo usuário seja sempre a primeira a ser desfeita[cite: 38].
* **(b) Processar pedidos de um restaurante em ordem:** **Fila (Queue)**. [cite_start]Necessita da política FIFO (First-In, First-Out) para garantir um atendimento justo e sequencial por ordem de chegada[cite: 39].
* **(c) Verificar se um arquivo HTML tem tags bem formadas:** **Pilha (Stack)**. À medida que o algoritmo faz o parse e encontra uma tag de abertura (ex: `<div>`), ela é empilhada. [cite_start]Ao encontrar uma de fechamento (`</div>`), ela obrigatoriamente precisa validar o casamento com o topo corrente da pilha[cite: 40].
* **(d) Navegar nos arquivos de um diretório em largura:** **Fila (Queue)**. [cite_start]O algoritmo de Busca em Largura (BFS) exige que todos os nós/arquivos de um mesmo nível de profundidade sejam guardados e processados ordenadamente antes de descer para o próximo nível[cite: 41].
* **(e) Verificar se uma sequência de palavras é palíndromo:** **Deque (Double-Ended Queue)**. [cite_start]Permite a remoção e comparação paralela das extremidades (início e fim) em tempo O(1), mantendo o fluxo limpo e eficiente[cite: 42].
