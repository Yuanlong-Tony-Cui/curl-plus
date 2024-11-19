# curl-plus

## Example Use Cases

### Handling Errors in URLs

`target/debug/curl-plus "www.eecg.toronto.edu"`

```
Requesting URL: www.eecg.toronto.edu
Method: GET
Error: The URL does not have a valid base protocol.
```

`target/debug/curl-plus "https://[...1]"`

```
Requesting URL: https://[...1]
Method: GET
Error: The URL contains an invalid IPv6 address.
```

`target/debug/curl-plus https://255.255.255.256`

```
Requesting URL: https://255.255.255.256
Method: GET
Error: The URL contains an invalid IPv4 address.
```

`target/debug/curl-plus "http://127.0.0.1:65536"`

```
Requesting URL: http://127.0.0.1:65536
Method: GET
Error: The URL contains an invalid port number.
```

### Handling Errors When Making Requests to the Web Server

`target/debug/curl-plus "https://example.rs"`

```
Requesting URL: https://example.rs
Method: GET
Error: Unable to connect to the server. Perhaps the network is offline or the server hostname cannot be resolved.
```

`target/debug/curl-plus "https://www.eecg.toronto.edu/~acui"`

```
Requesting URL: https://www.eecg.toronto.edu/~acui
Method: GET
Error: Request failed with status code: 404.
```

### Supporting the POST Method

`target/debug/curl-plus "https://jsonplaceholder.typicode.com/posts" -d "userId=1&title=Hello World" -X POST`

```
Requesting URL: https://jsonplaceholder.typicode.com/posts
Method: POST
Data: userId=1&title=Hello World
Response body (JSON with sorted keys):
{
  "id": 101,
  "title": "Hello World",
  "userId": "1"
}
```

### Sending JSON Formatted Data to a REST API Server

`target/debug/curl-plus --json '{"title": "World", "userId": 5}' "https://dummyjson.com/posts/add"`

```
Requesting URL: https://dummyjson.com/posts/add
Method: POST
JSON: {"title": "World", "userId": 5}
Response body (JSON with sorted keys):
{
  "id": 252,
  "title": "World",
  "userId": 5
}
```


`target/debug/curl-plus --json '{"title": "World"; "userId": 5}' "https://dummyjson.com/posts/add"`

```
Requesting URL: https://dummyjson.com/posts/add
Method: POST
JSON: {"title": "World"; "userId": 5}
thread 'main' panicked at src/handlers.rs:46:7:
Invalid JSON: Error("expected `,` or `}`", line: 1, column: 18)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```