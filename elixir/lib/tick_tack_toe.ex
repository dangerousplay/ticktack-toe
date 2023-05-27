
defmodule TicTacToe.State do
  defstruct board: %{}, current_player: "X"
end


defmodule TicTacToe do
  @board_size 9

  # Winning positions for the board, which has 9 slots:
  # %{
  #   1 => "X", 2 => "X", 3 => "X",
  #   4 => "",  5 => "O", 6 => "O",
  #   7 => "",  8 => "",  9 => ""
  # }
  @winning_positions [
     # Horizontal Lines
     [1, 2, 3], [4, 5, 6], [7, 8, 9],
     # Vertical Lines
     [1, 4, 7], [2, 5, 8], [3, 6, 9],
     # Diagonal Lines
     [1, 5, 9], [3, 5, 7]
   ]

  def start do
    IO.puts("Starting tic tac toe")
    IO.puts("")

    state = new_game_state()
    match_run(state)
  end

  defp match_run(state) do
    player = state.current_player
    board = state.board

    IO.puts("")

    display_board(board)

    IO.puts("")

    next_move = get_next_move(state.board, state.current_player)
    board = set_move(board, player, next_move)

    case check_winner(board) do
      :x ->
        IO.puts("Player X wins!")
        display_board(board)
        :ok
      :o ->
        IO.puts("Player O wins!")
        display_board(board)
        :ok
      :tie ->
        IO.puts("It's a tie!")
        :ok
      :continue ->
        next_state = %TicTacToe.State{board: board, current_player: next_player(player)}
        match_run(next_state)
    end
  end

  def display_board(board) do
    Map.values(board)
    |> Enum.map(fn v -> if String.length(v) < 1 do " " else v end end)
    |> Enum.chunk_every(3)
    |> Enum.each(fn row ->
      row |> Enum.join(" | ") |> IO.puts
      IO.puts("-----------")
    end)
  end

  def find_winner(board) do
    positions = Enum.map(@winning_positions, fn positions ->
      Enum.map(positions, fn p -> Map.get(board, p) end)
    end)

    positions
      |> Enum.filter(fn values -> !Enum.any?(values, fn v -> String.length(v) < 1 end) end)
      |> Enum.filter(fn values -> Enum.count(Enum.dedup(values)) == 1 end)
  end

  def check_winner(board) do
    winner_positions = find_winner(board)

    if Enum.count(winner_positions) > 0 do
      winner = List.flatten(winner_positions) |> Enum.at(0)
      case winner do
        "X" -> :x
        "O" -> :o
      end
    else
      occupied_slots = Map.filter(board, fn {_key, val} -> String.length(val) > 0 end)

      if Map.size(occupied_slots) == @board_size do
        :tie
      else
        :continue
      end
    end
  end

  def set_move(board, player, move) do
    Map.put(board, move, player)
  end

  def get_next_move(board, player) do
    IO.puts("")

    IO.puts("Player #{player}'s turn.")
    IO.puts("Enter a position (1-9): ")

    case IO.read(:line) do
      move ->
        move = move |> String.trim_trailing |> String.to_integer
        case validate_move(board, move) do
          :ok -> move
          :invalid_position ->
            IO.puts("Invalid move! Please enter a number between 1 and 9.")
            get_next_move(board, player)
          :position_taken ->
            IO.puts("Invalid move! The position is already taken.")
            get_next_move(board, player)
        end
    end
  end

  def validate_move(board, move) do
    if is_integer(move) and move in 1..9 do
      case Map.get(board, move) do
        "" -> :ok
        _ -> :position_taken
      end
    else
      :invalid_position
    end
  end

  def next_player("X"), do: "O"
  def next_player("O"), do: "X"

  def initialize_board do
    Enum.reduce(1..@board_size, %{}, fn pos, acc -> Map.put(acc, pos, "") end)
  end

  def new_game_state() do
    board = initialize_board()
    %TicTacToe.State{board: board}
  end
end


defmodule Main do
  def main do
    TicTacToe.start()
  end
end
