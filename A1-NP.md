# Estudo de Caso A1 — O Problema da Entrega Inteligente
# Otimização Logística em Plataformas Digitais de Delivery

**Aluna:** Vitória Leite
**Unidade Curricular:** Estruturas de Dados e Análise de Algoritmos (0006963)  
**Professor:** Alexandre "Montanha" de Oliveira  
**Data:** Junho/2026

---

## Questão 1 — Classificação do Problema (20 pontos)

### a) Classe de complexidade do problema

O problema de roteamento da FastBite pertence à classe **NP-Completo**.

**Justificativa:**

- **Classe P:** São problemas que podem ser resolvidos por um algoritmo determinístico em tempo polinomial O(n^k). O problema da FastBite não pertence a P porque não existe algoritmo conhecido que encontre a solução ótima em tempo polinomial para todas as instâncias.

- **Classe NP:** São problemas cuja solução pode ser **verificada** em tempo polinomial. O problema da FastBite está em NP porque, dada uma atribuição de pedidos e uma ordem de entrega para cada entregador, é possível verificar em tempo polinomial se a solução respeita todas as restrições (capacidade, janelas de tempo, prioridades) e calcular o tempo total.

- **Classe NP-Completo:** São problemas em NP para os quais qualquer outro problema em NP pode ser reduzido em tempo polinomial. O problema da FastBite é NP-Completo porque é uma generalização do Problema de Roteamento de Veículos (VRP), que por sua vez generaliza o Problema do Caixeiro Viajante (TSP) — e o TSP é um dos problemas NP-Completo clássicos. A restrição de 3 pedidos por entregador, prioridades e janelas de tempo não tornam o problema mais simples; pelo contrário, adicionam complexidade.

### b) Redução ao TSP

Para transformar uma instância do problema da FastBite em uma instância do TSP, podemos fazer a seguinte redução intuitiva:

1. **Caso base (1 entregador, capacidade infinita):** Com um único entregador e todos os pedidos, o problema se torna exatamente o TSP clássico: o entregador precisa visitar todos os restaurantes (coletas) e clientes (entregas) exatamente uma vez, retornando ao ponto inicial, minimizando a distância total percorrida. Cada pedido gera dois pontos de parada (coleta e entrega) que devem ser visitados em ordem (coleta antes da entrega), o que caracteriza o TSP com restrições de precedência.

2. **Caso geral (múltiplos entregadores):** Com vários entregadores, o problema é equivalente a múltiplos TSPs executados simultaneamente, onde cada entregador resolve seu próprio circuito. Isso corresponde ao VRP, que é uma extensão direta do TSP.

3. **Transformação formal simplificada:** Dado um conjunto de n pedidos com localizações de coleta C_i e entrega E_i, e m entregadores com posições iniciais P_j, podemos criar um grafo completo onde os vértices são todas as localizações envolvidas. O objetivo é particionar os vértices entre os m entregadores e, para cada entregador, encontrar uma ordem de visita que minimize o custo total. Esta é exatamente a definição do VRP, que se reduz ao TSP quando m=1.

### c) Inviabilidade da força bruta

**Cenário: 8 pedidos e 3 entregadores**

**Raciocínio combinatório:**

1. **Atribuição dos pedidos:** Cada pedido pode ser atribuído a qualquer um dos 3 entregadores. Para 8 pedidos, temos 3^8 = 6.561 possibilidades de atribuição.

2. **Ordenação por entregador (após atribuição):** Para cada entregador que recebe k pedidos:
   - O entregador precisa decidir a ordem de coleta e entrega
   - Com k pedidos, são 2k pontos (coleta + entrega de cada um)
   - Número de permutações: (2k)!
   - Porém, para cada pedido, a coleta deve vir antes da entrega, então precisamos considerar essa restrição
   - Aproximadamente: (2k)! / (2^k)

3. **Exemplo numérico:** Suponha uma distribuição típica onde E1 recebe 3 pedidos, E2 recebe 3 pedidos e E3 recebe 2 pedidos:
   - E1: (2×3)! / 2^3 = 720 / 8 = 90 ordens possíveis
   - E2: 90 ordens possíveis
   - E3: (2×2)! / 2^2 = 24 / 4 = 6 ordens possíveis
   - Total para esta atribuição: 90 × 90 × 6 = 48.600

4. **Total geral:** 48.600 (por atribuição) × 6.561 (atribuições possíveis) ≈ **318 milhões** de combinações

**Complexidade assintótica:** O(m^n × n!) onde m é o número de entregadores e n o número de pedidos. Este crescimento é **superexponencial** — dobrar o número de pedidos não apenas dobra as combinações, mas as eleva a um fator explosivo. Para 50 pedidos e 20 entregadores (cenário de pico), o número de combinações ultrapassa 10^80, muito além do número de átomos no universo.

---

## Questão 2 — Abordagem Gulosa (Greedy) (25 pontos)

### a) Funcionamento do algoritmo passo a passo

**Cenário de exemplo** com os dados fornecidos:

**Passo 1:** Listar todos os pedidos não atribuídos: P1, P2, P3, P4, P5.

**Passo 2 — Iteração 1:**
- Para cada pedido, calcular a distância de cada entregador disponível ao restaurante do pedido
- P1 (restaurante em 0,2): E1(1,1) = |1-0|+|1-2| = 2, E2(5,3) = 5+1=6, E3(7,6) = 7+4=11 → E1 é o mais próximo
- Atribuir P1 a E1. E1 agora tem 1/2 pedidos

**Passo 3 — Iteração 2:**
- P2 (restaurante em 1,1): E1(1,1) = 0, E2(5,3) = 5+2=7, E3(7,6) = 7+5=12 → E1 é o mais próximo
- Atribuir P2 a E1. E1 agora tem 2/2 pedidos (lotado)

**Passo 4 — Iteração 3:**
- P3 (restaurante em 4,4): E1 lotado, E2(5,3) = 1+1=2, E3(7,6) = 3+2=5 → E2 é o mais próximo
- Atribuir P3 a E2. E2 agora tem 1/2 pedidos

**Passo 5 — Iteração 4:**
- P4 (restaurante em 6,1): E1 lotado, E2(5,3) = 1+2=3, E3(7,6) = 1+5=6 → E2 mais próximo
- Atribuir P4 a E2. E2 agora tem 2/2 pedidos (lotado)

**Passo 6 — Iteração 5:**
- P5 (restaurante em 5,5): E1 e E2 lotados, E3(7,6) = 2+1=3
- Atribuir P5 a E3. E3 agora tem 1/3 pedidos

**Roteamento para cada entregador (sempre ir ao ponto mais próximo do atual):**
- E1 (posição inicial 1,1): P2(1,1) → P1(0,2) → entrega P2(7,2) → entrega P1(4,5)
- E2 (posição inicial 5,3): P3(4,4) → P4(6,1) → entrega P3(0,0) → entrega P4(3,6)
- E3 (posição inicial 7,6): P5(5,5) → entrega P5(8,1)

### b) Classificação como algoritmo guloso

O algoritmo é classificado como **guloso (greedy)** porque toma, a cada passo, a **melhor decisão local** sem considerar as consequências futuras.

A **propriedade de escolha local** aplicada é: "atribuir cada pedido ao entregador disponível mais próximo do restaurante, independentemente de como isso afetará as próximas atribuições". O algoritmo nunca reconsidera decisões anteriores — uma vez atribuído, o pedido não muda de entregador.

Da mesma forma, no roteamento, a escolha local é "sempre vá ao próximo ponto (coleta ou entrega) mais próximo da posição atual", sem planejar a rota completa.

### c) Contraexemplo da solução gulosa

**Cenário hipotético simples (adaptado dos dados):**

Considere apenas 2 pedidos e 2 entregadores:

- **P1:** Restaurante em (0,0), cliente em (10,0). Prioridade: Padrão
- **P2:** Restaurante em (1,1), cliente em (9,0). Prioridade: Urgente
- **E1:** Posição em (0,0), capacidade 2, carro rápido
- **E2:** Posição em (5,5), capacidade 2, carro rápido

**Solução gulosa:**
1. P1: E1 está em (0,0) = distância 0 ao restaurante P1, E2 está a distância |5-0|+|5-0| = 10 → E1 recebe P1
2. P2: E1 (já com P1, mas ainda com capacidade) está a 2 do restaurante P2, E2 está a |5-1|+|5-1| = 8 → E1 recebe P2

Ambos os pedidos vão para E1. Rota de E1: coleta P1(0,0) → coleta P2(1,1) → entrega P1(10,0) → entrega P2(9,0) = distância total aproximada: 1+2 + 9+1 + 1 = 14.

Porém, P2 é urgente e ficará esperando E1 fazer todo o trajeto até o cliente P1 antes de ser entregue.

**Solução ótima (que o guloso não encontra):**
- E1 pega P1 (já está no restaurante)
- E2 pega P2 (distância 8 até o restaurante, mas P2 é urgente e E1 já está ocupado com P1)

Rota E1: coleta P1(0,0) → entrega P1(10,0) = distância 10
Rota E2: desloca(5,5)→(1,1) → coleta P2(1,1) → entrega P2(9,0) = distância 4+2+8+1 = 15

Distância total: 25 (pior que a gulosa em distância), mas P2 (urgente) é entregue muito mais rápido. Se o objetivo é minimizar tempo de entrega respeitando prioridades, a solução gulosa foi subótima.

### d) Complexidade de tempo do algoritmo guloso

Para n pedidos e m entregadores:

- **Fase de atribuição:** Para cada pedido (n iterações), calculamos a distância para cada entregador (m comparações). Custo: **O(n × m)**
- **Fase de roteamento:** Para cada entregador com k pedidos, precisamos ordenar 2k pontos. Com abordagem gulosa (sempre ir ao mais próximo), cada decisão de próximo ponto requer percorrer os pontos restantes. Para k pedidos: O(k²). No total: O(Σ k²) que é **O(n²)** no pior caso.

**Complexidade total:** O(n × m + n²). Considerando que tipicamente n >> m, a complexidade dominante é **O(n²)** — polinomial, o que explica por que o algoritmo é rápido, mas potencialmente subótimo.

---

## Questão 3 — Programação Dinâmica e Divisão e Conquista (25 pontos)

### a) Aplicabilidade da Programação Dinâmica

**A PD é aplicável ao roteamento de um entregador com k pedidos**, pois o problema possui:

1. **Subestrutura ótima:** A rota ótima que passa por um conjunto de pontos e termina em um ponto específico contém dentro de si a rota ótima para o subconjunto de pontos restantes. Ou seja, se a melhor rota completa termina no ponto X, o trecho anterior (sem X) também deve ser ótimo.

2. **Sobreposição de subproblemas:** Diferentes sequências parciais podem chegar ao mesmo conjunto de pontos visitados, permitindo reutilização de cálculos.

**Definição informal do subproblema:**
- Estado: (conjunto de pontos já visitados, último ponto visitado)
- Valor: menor distância para visitar exatamente aquele conjunto e terminar naquele ponto
- Transição: para cada ponto não visitado, calcular o custo de ir do último ponto até ele

**Custo de memória e tempo:**
- Número de estados: 2^(2k) × (2k), onde 2k são os pontos de coleta e entrega
- Para k pedidos, são 2k pontos
- Memória: O(2^(2k) × k)
- Tempo: O(k² × 2^(2k))

**Limite prático para tempo real (≤ 2 segundos):**
Com uma implementação otimizada, PD consegue resolver instâncias de até aproximadamente **k = 5 a 7 pedidos** (10 a 14 pontos) dentro de 2 segundos. Para k = 8 pedidos (16 pontos), o número de estados é 2^16 × 16 ≈ 1.048.576, ainda factível mas no limite. Para k ≥ 10 (20 pontos), com 2^20 × 20 ≈ 20 milhões de estados, a PD se torna impraticável para execução em tempo real.

Como cada entregador da FastBite pode transportar até 3 pedidos (6 pontos), a PD seria **perfeitamente viável** para o roteamento individual — 2^6 × 6 = 384 estados, resolvido em microssegundos.

### b) Aplicabilidade da Divisão e Conquista

**É possível dividir o problema em subproblemas independentes?**

Sim, mas com restrições importantes. A divisão é possível quando:

1. Os pedidos são geograficamente separados por regiões distantes entre si
2. Não há sobreposição significativa entre as áreas de cobertura dos entregadores
3. As zonas são definidas de forma que um entregador não precise cruzar múltiplas zonas em uma mesma rota

**Exploração da divisão geográfica:**

A cidade pode ser dividida em quadrantes ou zonas (ex: norte, sul, leste, oeste, centro). A estratégia seria:

1. **Particionamento:** Atribuir cada pedido a uma zona baseada na localização do restaurante
2. **Alocação:** Designar entregadores a zonas específicas
3. **Resolução:** Resolver o problema de roteamento dentro de cada zona independentemente
4. **Combinação:** As soluções parciais formam a solução global

**Limitações:**

- **Fronteiras entre zonas:** Pedidos próximos às fronteiras podem ser mal atribuídos. Um entregador na zona A pode estar mais próximo de um restaurante na zona B do que qualquer entregador da zona B, mas não será considerado devido à divisão artificial.

- **Desequilíbrio de carga:** Algumas zonas podem ter muito mais pedidos que outras, gerando entregadores ociosos em algumas áreas e sobrecarregados em outras.

- **Pedidos cross-zone:** Um pedido com restaurante na zona A e cliente na zona B quebra o modelo de independência.

- **Natureza dinâmica:** Entregadores se movem e as condições mudam, tornando a divisão estática rapidamente obsoleta.

---

## Questão 4 — Comparação das Abordagens (15 pontos)

### Tabela Comparativa

| Critério | Greedy | Programação Dinâmica | Divisão e Conquista |
|----------|--------|---------------------|---------------------|
| **Qualidade da solução** | Baixa a média: soluções subótimas, pode falhar em casos com restrições complexas | Alta: encontra solução ótima para o subproblema de roteamento individual | Média: depende da qualidade da divisão; perde qualidade nas fronteiras |
| **Complexidade de tempo** | O(n² + n×m): muito rápido, escala bem | O(k² × 2^(2k)): exponencial, mas viável para k ≤ 7 | O(z × (n/z)²): polinomial se divisão for boa, onde z = número de zonas |
| **Complexidade de espaço** | O(n + m): mínimo, apenas armazena atribuições | O(2^(2k) × k): exponencial, inviável para k grande | O(n + m): similar ao greedy, armazena soluções parciais por zona |
| **Viabilidade em tempo real (≤ 2s)** | Totalmente viável, executa em ms | Viável apenas para roteamento individual com k ≤ 7 | Viável se zonas são pequenas o suficiente |
| **Escalabilidade com aumento de n** | Excelente: crescimento quadrático | Péssima: crescimento exponencial | Boa se zonas são redimensionadas proporcionalmente |
| **Facilidade de adaptação a mudanças** | Muito fácil: recalculável rapidamente | Difícil: precisa recomputar toda a tabela DP | Média: só a zona afetada precisa ser recalculada |

### Análise crítica e recomendação

Analisando as três abordagens no contexto operacional da FastBite — decisões em até 2 segundos, 80.000 pedidos por dia, picos de 50 pedidos simultâneos —, **nenhuma das abordagens puras é ideal isoladamente**, mas a **combinação de Divisão e Conquista com Greedy** emerge como a estratégia mais adequada.

A Programação Dinâmica, embora encontre a solução ótima para o roteamento individual, é impraticável como solução global devido ao custo exponencial. No entanto, é **extremamente valiosa como otimizador local**: como cada entregador transporta no máximo 3 pedidos, a PD resolve o roteamento individual em microssegundos, garantindo que cada entregador faça sua rota da melhor forma possível.

O Greedy puro é rápido mas produz soluções de baixa qualidade, especialmente quando há pedidos urgentes e restrições de prioridade.

A abordagem recomendada é um **híbrido**: Divisão e Conquista para particionar o problema geograficamente, Greedy para atribuição inicial rápida dentro de cada zona, e Programação Dinâmica para otimizar a rota de cada entregador individualmente. Esta combinação oferece o melhor equilíbrio entre velocidade e qualidade.

---

## Questão 5 — Solução de Engenharia Real (10 pontos)

### a) O que é uma heurística e por que é preferível

Uma **heurística** é uma técnica algorítmica que busca soluções suficientemente boas em tempo razoável, sem garantia de otimalidade. Diferentemente de um algoritmo exato, uma heurística "abre mão" da perfeição em troca de velocidade e escalabilidade.

**Por que heurísticas são preferíveis em sistemas como a FastBite:**

1. **Restrição de tempo real:** Com 2 segundos de limite e dezenas de pedidos simultâneos, algoritmos exatos são impossíveis
2. **Escala do problema:** 80.000 pedidos por dia tornam qualquer busca exaustiva inviável
3. **Ambiente dinâmico:** Condições mudam constantemente (trânsito, novos pedidos, entregadores ficando disponíveis), então uma solução "perfeita" para o momento atual rapidamente se torna obsoleta
4. **Custo-benefício:** A diferença entre uma solução 95% ótima (heurística) e uma 100% ótima (exata) é pequena em termos práticos, mas o custo computacional é ordens de grandeza diferente
5. **Tolerância a erros:** No contexto de delivery, pequenos atrasos são aceitáveis; o sistema não é de segurança crítica

### b) Estrutura de uma solução de engenharia real

Uma solução de engenharia real para a FastBite poderia ser estruturada em quatro camadas:

**Camada 1 — Particionamento geográfico (Pré-processamento):**
- Dividir a cidade em zonas/quadrantes dinâmicos baseados na densidade de pedidos em tempo real
- Cada zona opera como um "mini-problema" independente
- Zonas são ajustadas a cada 5-10 minutos conforme o fluxo de pedidos muda
- Entregadores são pré-alocados a zonas, mas podem cruzar fronteiras em situações específicas

**Camada 2 — Solução inicial gulosa (Atribuição rápida):**
- Dentro de cada zona, aplicar o algoritmo guloso: atribuir cada pedido ao entregador mais próximo do restaurante
- Esta fase é executada primeiro e garante uma solução baseline em milissegundos
- Serve como ponto de partida para refinamentos

**Camada 3 — Refinamento local (Otimização iterativa):**
- Aplicar técnicas de busca local: trocar dois pedidos entre dois entregadores e verificar se o tempo total diminui
- Tentar reassignar pedidos de fronteira entre zonas vizinhas
- Para cada entregador, usar Programação Dinâmica (já que k ≤ 3) para encontrar a rota ótima individual
- Executar quantas iterações de refinamento forem possíveis dentro do tempo restante

**Camada 4 — Limite de tempo estrito (Safety net):**
- Definir um timeout de 1.8 segundos (dos 2 segundos disponíveis)
- Se o refinamento não terminou, retornar a melhor solução encontrada até o momento
- A solução inicial gulosa garante que sempre há uma resposta válida, mesmo que não otimizada

### c) Quando vale a pena buscar a solução ótima

Vale a pena buscar a solução ótima em situações onde:

1. **Planejamento offline:** Definir zonas de entrega, posicionamento de dark kitchens ou hubs de distribuição — decisões estratégicas que são calculadas uma vez e valem por meses
2. **Escala reduzida:** Madrugadas com poucos pedidos (3-5 pedidos simultâneos), onde a solução exata é computacionalmente viável
3. **Benchmarking:** Calcular periodicamente a solução ótima para uma amostra pequena de ciclos e comparar com a solução heurística, medindo a qualidade ao longo do tempo
4. **Entregas especiais:** Pedidos de alto valor (como medicamentos ou cargas premium) onde o custo do atraso justifica o custo computacional de uma solução exata

---

## Questão 6 — Reflexão Crítica (5 pontos)

"Bom o suficiente" é a melhor decisão técnica quando o custo de buscar a perfeição ultrapassa o benefício prático de alcançá-la. No contexto da FastBite, uma solução 100% ótima exigiria tempo exponencial — literalmente impossível para 50 pedidos simultâneos dentro de 2 segundos. Mesmo que fosse possível, o ambiente é dinâmico: enquanto o algoritmo calcula a rota perfeita, o trânsito muda, novos pedidos chegam e a solução "perfeita" já nasce desatualizada.

A complexidade computacional nos ensina que certos problemas são intrinsecamente difíceis — não por falta de engenhosidade, mas por limites fundamentais da computação. Ignorar isso e insistir na solução ótima não é perfeccionismo técnico, é negligência com os requisitos reais do sistema.

Um engenheiro eficaz entende que seu papel não é encontrar a resposta matemática ideal, mas sim a melhor solução possível dentro das restrições do mundo real: tempo, recursos computacionais, dinheiro e expectativas dos usuários. Quando uma heurística entrega 95% da qualidade ótima em 0,001% do tempo, insistir na solução exata é a decisão tecnicamente inferior.

---

## Referência aos Dados do Cenário

Os dados fornecidos foram utilizados para ilustrar a aplicação do algoritmo guloso na Questão 2, demonstrando passo a passo como a heurística funciona com os 5 pedidos e 3 entregadores do cenário. A distância Manhattan foi aplicada conforme especificado, e as restrições de capacidade (2-3 pedidos por entregador) foram respeitadas durante a demonstração.

---

**Fim do documento.**
