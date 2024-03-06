defmodule GenTcp.Native do
  use Rustler, otp_app: :gen_tcp, crate: "gen_tcp_native"

  # When your NIF is loaded, it will override this function.
  def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
  def serve(_router, _text), do: :erlang.nif_error(:nif_not_loaded)
end
