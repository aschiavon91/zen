defmodule ZenEngine do
  use Rustler,
    otp_app: :zen_engine,
    crate: :zen_elixir

  def evaluate_expression(_exp, _ctx), do: :erlang.nif_error(:nif_not_loaded)
  def evaluate_unary_expression(_exp, _ctx), do: :erlang.nif_error(:nif_not_loaded)
  def test_fnc(_fnc), do: :erlang.nif_error(:nif_not_loaded)
end
