# Elixir Tic Tac Toe

**Tic tac toe** implementation using functional programming paradigm in Elixir.

## Running

You need `mix` and `elixir` to compile and run the application.
You can follow the instructions in [https://elixir-lang.org/install.html](https://elixir-lang.org/install.html) to install them.

To execute the application you can use `mix`:

```shell
mix compile
mix run -e Main.main
```

## Running the automated tests

```shell
mix compile
mix test
```

## Example of match

```text
Starting tic tac toe


  |   |  
-----------
  |   |  
-----------
  |   |  
-----------


Player X's turn.
Enter a position (1-9): 
1

X |   |  
-----------
  |   |  
-----------
  |   |  
-----------


Player O's turn.
Enter a position (1-9): 
2

X | O |  
-----------
  |   |  
-----------
  |   |  
-----------


Player X's turn.
Enter a position (1-9): 
4

X | O |  
-----------
X |   |  
-----------
  |   |  
-----------


Player O's turn.
Enter a position (1-9): 
5

X | O |  
-----------
X | O |  
-----------
  |   |  
-----------


Player X's turn.
Enter a position (1-9): 
7
Player X wins!
X | O |  
-----------
X | O |  
-----------
X |   |  
-----------

```

## Implementation details

The implementation tries to use in the best way the functional concepts.

Specifically it uses:

### Imutability and pure functions

Immutability, means that once a value is assigned, it cannot be modified.
In Elixir, **data structures** are **immutable** by default.
The code utilizes **immutability** by using **immutable** data structures such as maps to represent the **game board** and updating them using functions like `Map.put/3`.

Example:

```elixir
def set_move(board, player, move) do
  Map.put(board, move, player)
end
```

In the `set_move/3` function, a new map is created by adding a move made by a player to the board.
The original board remains unchanged, and the updated board is returned as a new value.

The `set_move/3` is also a **pure function** which means it does not have **side effects** and always produce the **same output** for the **same inputs**.
It relies **only** on its input parameters to compute the result and do not modify any external state ensuring predictable behavior and facilitating testing.


### Pattern Matching

**Pattern matching** is a powerful feature in Elixir that allows you to match values against **patterns**.
It is used extensively throughout the code to handle different cases and make decisions based on the **matched patterns**.
**Pattern matching** is particularly useful in Elixir for extracting values from **data structures**.

Example:

```elixir
def next_player("X"), do: "O"
def next_player("O"), do: "X"
```

The `next_player/1` function uses pattern **matching** to determine the next player based on the current player.
If the current player is "X," the function returns "O," and vice versa.
This **pattern matching** simplifies the logic of switching players.


### Recursion

**Recursion** is a fundamental concept in functional programming, where a function calls itself to solve a problem by breaking it down into smaller subproblems.
The code utilizes **recursion** in the `match_run/1` function to continue the game until there is a **winner** or a **tie**.

Example:

```elixir
defp match_run(state) do
  # ...
  case check_winner(board) do
      # ...
      :continue ->
          next_state = %TicTacToe.State{board: board, current_player: next_player(player)}
          match_run(next_state)
  end
end
```

The `match_run/1` function calls itself recursively with the updated game state (`next_state`) as the input when the game is ongoing (`:continue`).
This recursive call allows the game to progress until a **winner** or a **tie** is achieved.

### Higher-Order Functions and Pipelines

Elixir supports **higher-order functions**, which are functions that can accept other functions as arguments or return them as results.
The code utilizes **higher-order functions** and the pipeline operator (`|>`) to compose data transformations and achieve a more concise and readable code structure.

Example:

```elixir
def display_board(board) do
    Map.values(board)
    |> Enum.map(fn v -> if String.length(v) < 1 do " " else v end end)
    |> Enum.chunk_every(3)
    |> Enum.each(fn row ->
        row |> Enum.join(" | ") |> IO.puts
        IO.puts("-----------")
    end)
end
```

The `display_board/1` function uses **higher-order functions** and pipelines to transform the board data structure.
It extracts the values from the board map, replaces empty strings with spaces, groups the values into rows, and then prints the formatted board to the console.

### Automated tests

Due to the use of functional concepts such as **imutability**, **pure functions**, the automated tests are more concise and simple.

This test validates the `check_winner/1` game logic by providing a board that is **imutable** and the function returns the **winner**, a **tie** or **continue**:

```elixir
test "Player X wins with a horizontal line" do
  board = %{
    1 => "X", 2 => "X", 3 => "X",
    4 => "", 5 => "O", 6 => "O",
    7 => "", 8 => "", 9 => ""
  }

  actual_winner = TicTacToe.check_winner(board)
  assert actual_winner == :x
end
```

