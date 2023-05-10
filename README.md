Na aula de hoje os integrantes do grupo deverão escolher um paradigma, apresentar sua
proposta de software para o professor, e formalizá-la em um documento descrevendo tanto as
funcionalidades que o software possuirá, quanto o motivo pela qual esse paradigma seria
adequado. A postagem deverá ser feita até as 22:30.


# Proposta

Desenvolver o jogo da velha utilizando o paradigma **funcional** na Linguagem **Elixir** e outra implementação utilizando o paradigma **imperativo** na linguagem **Rust**.
A implementação em **Elixir** utilizaria de todos os recursos funcionais da linguagem, como **imutabilidade**.
A implementação em **Rust** utilizaria das funcionalidades que existem na linguagem para construir um código com mais **mutabilidade**, **estado global**, **side effects**.

## Motivadores

A linguagem Elixir por ter mais recursos de uma linguagem funcional pode facilitar a testabilidade e o controle de estado do jogo, devido:
- **Imutabilidade**: É possível implementar o estado do jogo de forma imutável, onde conseguimos representar o estado do 
jogo em um único objeto imutável, onde a modificação do mesmo gera outro objeto.
- **Pattern Matching**: facilita o desenvolvimento de máquinas de estado, permitindo mais facilmente definir as transições 
entre os estados do jogo.
- **No side effects**: evitando o uso de **side effects** nas variáveis e estados globais, facilita o teste unitário. 

