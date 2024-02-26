defmodule ZenEngineTest do
  use ExUnit.Case
  doctest ZenEngine

  test "greets the world" do
    assert ZenEngine.hello() == :world
  end
end
