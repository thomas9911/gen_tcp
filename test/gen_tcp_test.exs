defmodule GenTcpTest do
  use ExUnit.Case
  doctest GenTcp

  defp do_recv(socket, data) do
    case :gen_tcp.recv(socket, 0) do
      {:ok, body} ->
        do_recv(socket, [data, body])
      {:error, :closed} ->
          {:ok, :erlang.list_to_binary(data)}
    end
  end

  test "erlang gen_tcp" do
    {:ok, listen_socket} = :gen_tcp.listen(1234, [])
    case gen_tcp.accept(port) do
      {:ok, response_socket} ->
          {:ok, data} = do_recv(socket, [])
          :ok = :gen_tcp.close(socket)
          :ok = :gen_tcp.close(listen_socket)

      error -> raise inspect(error)
    end

  end
end
