# Simple multi-threaded HTTP server (WIP)

This is a simple multi-threaded HTTP server. This was written as an exercise to better understand how http server library is implemented under the hood. The project is still a work in progress.

You can run the project with `cargo run`.

## Info

The server is multi-threaded. A pool of threads is managed by the Workerpool.

The server currently supports
- GET, POST, PUT, DELETE methods
- A basic RouteHandler trait `HttpRequest -> HttpResponse`
- Query parameters and requests with a body

The HttpRequest has the following model

```
pub struct HttpRequest {
    pub method: HttpMethod,
    pub target: String,
    pub version: HttpVersion,
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub query_params: HashMap<String, String>,
}
```

## How to use the server

In src/services/example_services.rs you will find examples of some simple HTTP services and how to use the RouteHandler trait to create HTTP service. In src/main.rs you can see an example of how the services are registed to the server builder. By default the HttpServerBuilder has 16 threads. You can increase this with worker_pool_limit method.

## TODO

- Route parameters (currently WIP): support registering routes like /user/{id} for example
- Add support for some less commonly used HTTP methods
- Reduce boilerplate of setting up services (perhaps macros?)
