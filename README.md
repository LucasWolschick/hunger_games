# Solução do desafio de Jogos Vorazes

Lucas Wolschick, 2021

Este repositório contém a solução de um problema proposto em um evento de Matemática. Em suma, são dados uma lista de itens de sobrevivência e seus detalhes, juntamente com um mapa em grade contendo a disposição deles em um tabuleiro.

Partindo do ponto S no mapa dado e movendo-se apenas horizontalmente ou verticalmente, sendo possível coletar itens quando você se encontra no espaço deles, quais são os itens que, quando coletados, possuem a maior pontuação de sobrevivência possível em apenas 30 movimentos e com um peso máximo de 30 kg?

Este repositório resolve este problema de dois modos: por meio da [Programação dinâmica](https://pt.wikipedia.org/wiki/Programação_dinâmica), e por uma busca exaustiva de todas as possibilidades sensíveis.

## Executando

Para executar as soluções, é necessário possuir instalado a versão mais recente da linguagem de programação [Rust](https://www.rust-lang.org), além de possuir `cargo` no seu PATH.

Navegue até o diretório contendo esse repositório em uma linha de comando e execute:

```bash
$ cargo run --release --bin analytical
# ou
$ cargo run --release --bin physical
```

O executável `analytical` faz a busca exaustiva de todas as possibilidades, e o executável `physical` utiliza a programação dinâmica para obter o resultado. O programa deve retornar a lista de posições da solução ideal, juntamente com o estado em cada passo da solução.

## Soluções

### 1 - Busca exaustiva (`--bin analytical`)

Este método tenta maximizar o valor de sobrevivência total testando todas as 2^20 possibilidades de itens, enquanto mantém a equação de peso total abaixo ou igual a 30. A princípio, ignoramos a questão das distâncias entre os itens.

Sejam as variáveis `x_1, x_2, x_3, ..., x_20` representantes de cada um dos itens respectivamente, e podendo valer 0 (quando não se coleta o item) ou 1 (quando o item é coletado). Então a expressão que representa a pontuação total ao fim será dada por

`V(x_1, x_2, ..., x_20) = 15x_1 + 14x_2 + 10x_3 + ... + 25x_20`,

e a expressão que representa o peso total dos itens será dada por

`P(x_1, x_2, ..., x_20) = 9x_1 + 5x_2 + 3x_3 + ... + 12x_20`.

Em seguida, concatenamos os valores das variáveis em série. Por exemplo, para x_1 = 1, x_2 = 1 e x_3 = 0, o resultado seria `011`. Ao fim, teremos uma série de 20 números 0 ou 1, que podem ser tratados como apenas um número em base 2; como qualquer combinação de 20 números é possível, então todos os números de 2^0-1 = 0b0000000000000000000000000000 a 2^20-1 = 0b1111111111111111111111111111 são representações de escolhas válidas para o nosso problema.

Com uma representação concisa de nossas escolhas, e funções que retornam a pontuação e o custo para elas, podemos instruir o computador a verificar quais números de 2^0-1 a 2^20-1 (aprox. 1 milhão de possibilidades), quando convertidos às variáveis, resultam em peso menor do que o limite, e depois podemos colocar as pontuações obtidas dos números válidos em ordem.

Em seguida, verificamos se cada possibilidade pode ser alcançada em tempo adequado. Criamos uma cópia do mapa original contendo apenas os itens da solução, e estabelecemos se existe um caminho que passa por todos eles em tempo hábil. Esse caminho é calculado através do método recursivo estabelecido na seção sobre a solução em programação dinâmica.

Passando por todas as possibilidades em ordem decrescente de pontuação, paramos na primeira possibilidade válida e retornamos esse caminho, que será o melhor por consequência.

Este método só é possível em pouco tempo pois a quantidade de itens é pequena; uma análise em 200 itens seria muito mais lenta, e nesse caso uma solução aproximada seria mais vantajosa.

### 2 - Programação dinâmica (`--bin physical`)

Programação dinâmica é um método de otimização matemática e de programação que envolve a quebra de decisões complicadas em decisões mais simples, baseada no Princípio de Otimalidade de Bellman, que diz em termos grosseiros que *um processo otimizado para a solução de um problema é uniformemente ótimo: ele possui comportamento otimizado independente do estado e decisões iniciais*. Isso pode ser exemplificado, nas palavras de Dijkstra, como "Se R é um nó no caminho mínimo de P até Q, o conhecimento deste implica conhecimento do caminho mínimo de P até R".

Com base nisso, e levando o nosso problema em consideração, percebemos que:

1. Se o caminho que nos leva à melhor solução partindo de A passa pelos pontos A, B, C, D, e E nessa ordem, então o caminho que nos leva à melhor solução partindo de B passa pelos pontos B, C, D e E. Caso contrário, haveria um modo melhor de fazer o caminho de A até E também, e esse não seria o melhor caminho.

2. A melhor decisão a ser tomada é aquela que maximiza a nossa pontuação de sobrevivência.

Precisamos estabelecer, portanto, uma função recursiva que explore todas as possibilidades e retorne a com maior pontuação, da forma `f(posição, tempo_restante, peso_restante, pontos_até_agora) -> pontuação`. A função também recebe os itens disponíveis naquela iteração, mas isso é omitido neste texto por brevidade. Por exemplo, se o melhor caminho possível passa pelos pontos A, B, C e D, então a árvore de invocações seria:

1. *f(A, 30, 30, 0)* -> **?**

2. f(A, 30, 30, 0) -> *f(B, 30 - distAB, 30 - pesoA, valorA)* -> **?**

3. f(A, 30, 30, 0) -> f(B, 30 - distAB, 30 - pesoA, valorA) -> *f(C, 30 - distAB - distBC, 30 - pesoA - pesoB, valorA + valorB)* -> **?**

4. f(A, 30, 30, 0) -> f(B, 30 - distAB, 30 - pesoA, valorA) -> f(C, 30 - distAB - distBC, 30 - pesoA - pesoB, valorA + valorB) -> *f(D, 30 - distAB - distBC - distCD, 30 - pesoA - pesoB - pesoC, valorA + valorB + valorC)* -> **valorA + valorB + valorC + valorD (não há mais destinos válidos a partir de D)**

5. f(A, 30, 30, 0) -> f(B, 30 - distAB, 30 - pesoA, valorA) -> *f(C, 30 - distAB - distBC, 30 - pesoA - pesoB, valorA + valorB)* -> **valorA + valorB + valorC + valorD**

6. f(A, 30, 30, 0) -> *f(B, 30 - distAB, 30 - pesoA, valorA)* -> **valorA + valorB + valorC + valorD**

7. *f(A, 30, 30, 0)* -> **valorA + valorB + valorC + valorD**

8. **valorA + valorB + valorC + valorD**

A função recursiva invoca a si mesma para todos os nós alcançáveis e retorna a melhor pontuação. Se em algum momento do desenrolar não houvesse lugar válido para se mover, seja por falta de tempo ou de espaço, a pontuação acumulada até então seria retornada.

Com algumas modificações, a função pode retornar o caminho percorrido até então, para visualização e conferência dos dados. A implementação final da função está localizada em `src/pathfind.rs`.

Portanto, nesse método, coletamos todos os itens dispostos no mapa e os damos à função, que nos retorna o melhor caminho. Além disso, se nós soubermos a lista de itens de uma solução, também podemos utilizar esta função para verificar se a solução possui caminho válido.

## Adendo

Os dados utilizados pelo código estão presentes na pasta `data`:

* items.csv é uma tabela em formato csv contendo os dados dos itens. É necessário que as entradas Weight, Value e Item existam para todos os itens.

* map.txt é um mapa contendo todos os itens nas suas respectivas posições. Números indicam índices de itens, pontos indicam células vazias e espaço em branco é ignorado.

As rotinas de processamento desses dados não são robustas e o código não deve ser utilizado no tratamento de dados inseguros.

## Licença

Este programa é distribuído sob a licença GNU General Public License 3. A licença pode ser verificada em `COPYING`.

## Contribuições

Este projeto não está aceitando contribuições.
