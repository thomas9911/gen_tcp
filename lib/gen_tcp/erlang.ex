defmodule GenTcp.Erlang do
  defp do_recv(socket, data) do
    case :gen_tcp.recv(socket, 16) do
      {:ok, body} ->
        IO.inspect(body)
        if :lists.suffix('\r\n\r\n', body) do
          {:ok, :erlang.list_to_binary([data, body])}
        else
          do_recv(socket, [data, body])
        end
      {:error, :closed} ->
          {:ok, :erlang.list_to_binary(data)}

      e -> e
    end
  end
  
  def run_server_once do
    {:ok, listen_socket} = :gen_tcp.listen(1234, [active: false])
    case :gen_tcp.accept(listen_socket) do
      {:ok, response_socket} ->
          resp = do_recv(response_socket, [])
          response = 'HTTP/1.1 200 OK\r\n\r\n'
          :gen_tcp.send(response_socket, response)

          :ok = :gen_tcp.close(response_socket)
          :ok = :gen_tcp.close(listen_socket)
          resp
  
      error -> 
        :ok = :gen_tcp.close(listen_socket)
        
        {:error,inspect(error)}
    end
  end

end
