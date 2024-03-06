router = %GenTcp.Router{
  routes: [
    %GenTcp.Router.Route{method: :get, path: "/", response: "Hallo!! wave"},
    %GenTcp.Router.Route{method: :get, path: "/api/:id", response: "API at {{ id }}"}
  ]
}

GenTcp.Native.serve(router, "hallof from elixir and rust !!")
