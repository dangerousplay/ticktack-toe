defmodule ElixirTickTackToeTest do
  use ExUnit.Case
  doctest TicTacToe

#  test "greets the world" do
#    TicTacToe.start()
#  end

  test "Validate move - position taken" do
    board = %{
      1 => "X", 2 => "O", 3 => "",
      4 => "", 5 => "", 6 => "",
      7 => "", 8 => "", 9 => ""
    }

    move = 1

    actual = TicTacToe.validate_move(board, move)
    assert actual == :position_taken
  end

  test "Validate move - position out of range" do
    board = %{
      1 => "X", 2 => "O", 3 => "",
      4 => "", 5 => "", 6 => "",
      7 => "", 8 => "", 9 => ""
    }

    move = 10

    actual = TicTacToe.validate_move(board, move)
    assert actual == :invalid_position
  end

  test "Validate move - valid position" do
    board = %{
      1 => "X", 2 => "O", 3 => "",
      4 => "", 5 => "", 6 => "",
      7 => "", 8 => "", 9 => ""
    }

    move = 3

    actual = TicTacToe.validate_move(board, move)
    assert actual == :ok
  end


  test "Player X wins with a horizontal line" do
    board = %{
      1 => "X", 2 => "X", 3 => "X",
      4 => "", 5 => "O", 6 => "O",
      7 => "", 8 => "", 9 => ""
    }

    actual_winner = TicTacToe.check_winner(board)
    assert actual_winner == :x
  end

  test "Player O wins with a diagonal line" do
    board = %{
      1 => "O", 2 => "X", 3 => "X",
      4 => "", 5 => "O", 6 => "X",
      7 => "O", 8 => "", 9 => "O"
    }

    actual_winner = TicTacToe.check_winner(board)
    assert actual_winner == :o
  end

  test "Game ends in a tie" do
    board = %{
      1 => "X", 2 => "O", 3 => "X",
      4 => "X", 5 => "O", 6 => "O",
      7 => "O", 8 => "X", 9 => "O"
    }

    actual_winner = TicTacToe.check_winner(board)
    assert actual_winner == :tie
  end

  test "Game continues when there is not a winner or tie" do
    board = %{
      1 => "X", 2 => "O", 3 => "",
      4 => "", 5 => "O", 6 => "",
      7 => "", 8 => "", 9 => ""
    }

    actual_winner = TicTacToe.check_winner(board)
    assert actual_winner == :continue
  end

  test "Player X wins with a vertical line" do
    board = %{
      1 => "X", 2 => "O", 3 => "",
      4 => "X", 5 => "O", 6 => "",
      7 => "X", 8 => "", 9 => ""
    }

    actual_winner = TicTacToe.check_winner(board)
    assert actual_winner == :x
  end


end
