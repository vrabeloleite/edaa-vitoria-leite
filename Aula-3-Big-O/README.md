# Resolução da Lista: Análise de Complexidade (Big-O)

**Estudante:** Vitória Leite  
**Matéria:** Estrutura de Dados e Análise de Algoritmos  

---

### Exercício 2
* **Complexidade:** $O(n)$
* **Justificativa:** O código usa um laço para percorrer os elementos uma vez. O número de operações cresce de forma linear, na mesma proporção do tamanho da entrada $n$.

---

### Exercício 3 (Busca Binária)
* **Complexidade:** $O(\log n)$
* **Justificativa:** A cada repetição do laço `while`, o algoritmo corta o espaço de busca pela metade ao calcular o ponto médio. Essa divisão sucessiva por 2 faz com que o tempo de execução cresça de forma logarítmica.

---

### Exercício 4 (Pares com soma)
* **Complexidade:** $O(n^2)$
* **Justificativa:** O algoritmo tem dois laços `for` aninhados (um dentro do outro). O laço externo roda $n$ vezes e o interno faz as combinações. No pior caso, essa comparação de elementos par a par gera um crescimento de tempo quadrático.

---

### Exercício 5 (Imprimir pares e soma)
* **Complexidade:** $O(n^2)$
* **Justificativa:** O código é dividido em duas partes independentes. O Bloco 1 é linear, ou seja, $O(n)$. Já o Bloco 2 tem dois loops aninhados, o que dá $O(n^2)$. Como a gente sempre considera o pior caso (o termo dominante), a complexidade final do algoritmo fica sendo $O(n^2)$.

---

### Exercício 6 (Potências de dois)
* **Complexidade:** $O(\log n)$
* **Justificativa:** No laço `while`, a variável `i` dobra de valor a cada iteração porque é multiplicada por 2 (`i *= 2`). Como o crescimento é geométrico e não de um em um, o número de passos para chegar até o limite $n$ é logarítmico.

---

### Exercício 7 (Fibonacci Recursivo)
* **Complexidade:** $O(2^n)$
* **Justificativa:** A função faz duas chamadas recursivas para ela mesma a cada execução. Isso cria uma árvore de chamadas que dobra de tamanho a cada nível que desce na recursão, gerando um crescimento de tempo exponencial.

---

### Exercício 8 (Ordenação Bolha / Bubble Sort)
* **Complexidade:** $O(n^2)$
* **Justificativa:** O algoritmo usa dois loops `for` aninhados para comparar os elementos vizinhos e fazer as trocas. Mesmo diminuindo o tamanho do laço de dentro a cada passada (`n - i - 1`), a lógica no pior e no médio caso continua sendo quadrática.

---

### Exercício 9 (Produto de Matrizes)
* **Complexidade:** $O(n^3)$
* **Justificativa:** O algoritmo usa três laços `for` um dentro do outro para fazer o cálculo. Como cada um dos loops roda até o tamanho $n$, a operação mais interna acaba sendo executada exatamente $n \times n \times n = n^3$ vezes, o que caracteriza um tempo cúbico.

---

### Exercício 10 (Merge Sort)
* **Complexidade:** $O(n \log n)$
* **Justificativa:** O algoritmo usa a estratégia de divisão e conquista. A parte que divide a lista ao meio repetidamente por recursão gera uma árvore de altura $\log n$. Depois, a parte que junta e ordena as sublistas (os laços `while` e os `extend`) passa pelos elementos levando tempo linear $O(n)$. Juntando as duas etapas, o custo total fica em $O(n \log n)$.
