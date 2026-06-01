# Atividade: Reescrita de Algoritmos em Rust
**Aluno:** Vitória Rabelo Leite  
**Matéria:** Estruturas de Dados e Análise de Algoritmos  
**Professor:** Alexandre Montanha  

---

## Exercício 1 — Verificar Primeiro
**Complexidade:** O(1)  
**Lógica do algoritmo:** Pega uma lista e vê se tá vazia. Se não tiver, retorna o elemento que tá no começo (índice 0).  
**Justificativa da complexidade:** Só faz uma checagem rápida e acessa direto a posição do vetor. Não tem nenhum loop aqui, então roda em tempo constante.

## Exercício 2 — Somar Lista
**Complexidade:** O(n)  
**Lógica do algoritmo:** Cria uma variável acumuladora e passa por cada item da lista somando todo mundo dentro de um for.  
**Justificativa da complexidade:** Como tem um loop for simples que passa por todos os n elementos da lista uma vez só, o tempo cresce junto com o tamanho da entrada.

## Exercício 3 — Busca Binária
**Complexidade:** O(log n)  
**Lógica do algoritmo:** Pega uma lista ordenada e vai cortando ela no meio. Olha se o alvo tá no meio, se for menor vai pra esquerda, se for maior vai pra direita.  
**Justificativa da complexidade:** A cada iteração do while o tamanho do problema cai pela metade. Isso divide a busca por 2 várias vezes, caindo na classe logarítmica.

## Exercício 4 — Pares com Soma
**Complexidade:** O(n^2)  
**Lógica do algoritmo:** Usa dois loops para comparar cada número da lista com todos os outros da frente e ver se a soma dá o número alvo.  
**Justificativa da complexidade:** São dois loops for aninhados (um dentro do outro). O de fora roda n vezes e o de dentro roda o resto, gerando uma complexidade quadrática.

## Exercício 5 — Imprimir Pares e Pares
**Complexidade:** O(n^2)  
**Lógica do algoritmo:** Primeiro roda um loop para printar cada número sozinho, e depois roda outro bloco com dois loops colados para printar todas as combinações de pares.  
**Justificativa da complexidade:** O primeiro pedaço é O(n), mas o segundo pedaço tem dois loops grudados que dão O(n^2). Pela regra da soma do Big-O, o termo maior (n^2) manda em tudo.

## Exercício 6 — Potências de Dois
**Complexidade:** O(log n)  
**Lógica do algoritmo:** Começa com o valor 1 e vai multiplicando ele por 2 enquanto for menor que o número limite n passado na função.  
**Justificativa da complexidade:** O loop vai multiplicando a variável por dois a cada passo (crescimento exponencial). Isso faz o loop acabar bem rápido, em uma taxa de log n.

## Exercício 7 — Fibonacci Recursivo
**Complexidade:** O(2^n)  
**Lógica do algoritmo:** Calcula o Fibonacci chamando a própria função duas vezes (para n-1 e n-2) até bater no caso base.  
**Justificativa da complexidade:** Cada chamada da função abre mais duas chamadas novas. Isso cria uma árvore gigante de execução que dobra de tamanho a cada nível, sendo exponencial.

## Exercício 8 — Ordenação Bolha (Bubble Sort)
**Complexidade:** O(n^2)  
**Lógica do algoritmo:** Varre o vetor comparando vizinhos. Se o da esquerda for maior que o da direita, troca eles de lugar usando o swap. Repete isso até ordenar tudo.  
**Justificativa da complexidade:** Usa dois loops encadeados para fazer as varreduras e ir jogando os maiores elementos para o fim, o que cai no pior caso em O(n^2).

## Exercício 9 — Produto de Matrizes
**Complexidade:** O(n^3)  
**Lógica do algoritmo:** Faz a multiplicação tradicional de duas matrizes quadradas fazendo linha vezes coluna e somando no resultado.  
**Justificativa da complexidade:** Para calcular o produto das matrizes n por n, o algoritmo precisa rodar três loops for um dentro do outro, multiplicando n x n x n.

## Exercício 10 — Merge Sort
**Complexidade:** O(n log n)  
**Lógica do algoritmo:** Divide o vetor na metade jogando para a recursão, e depois junta as partes de volta de forma ordenada usando uma função auxiliar de merge.  
**Justificativa da complexidade:** A parte de dividir o vetor repetidamente gera uma altura de log n na árvore de execução, e o processo de juntar (merge) custa O(n) em cada nível. Multiplicando os dois fica n log n.
