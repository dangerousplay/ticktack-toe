
# Tick Tack Toe

<!-- TOC -->
* [Tick Tack Toe](#tick-tack-toe)
* [Exercício](#exercício)
* [Proposta](#proposta)
  * [Motivadores](#motivadores)
<!-- TOC -->

# Exercício

Os integrantes do grupo deverão escolher um paradigma, apresentar sua
proposta de software para o professor, e formalizá-la em um documento descrevendo tanto as
funcionalidades que o software possuirá, quanto o motivo pela qual esse paradigma seria
adequado.


# Proposta

Desenvolver o jogo da velha utilizando o paradigma **funcional** na linguagem **Elixir** e outra implementação utilizando o paradigma **imperativo** na linguagem **Rust**.
A implementação em **Elixir** utilizaria de todos os recursos funcionais da linguagem, como **imutabilidade**.
A implementação em **Rust** utilizaria das funcionalidades que existem na linguagem para construir um código com mais **mutabilidade**, **estado global**, **side effects**.

## Motivadores

A linguagem Elixir por ter mais recursos de uma linguagem funcional pode facilitar a testabilidade e o controle de estado do jogo, devido:
- **Imutabilidade**: é possível implementar o estado do jogo de forma imutável, onde conseguimos representar o estado do 
jogo em um único objeto imutável, onde a modificação do mesmo gera outro objeto.
- **Pattern Matching**: facilita o desenvolvimento de máquinas de estado, permitindo mais facilmente definir as transições 
entre os estados do jogo.
- **No side effects**: evita os **side effects** nas variáveis e estados globais, facilitando o teste unitário e a predictability da execução do código.

