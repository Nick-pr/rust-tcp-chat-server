This is a simple TCP chat sever I implemented in Rust to get familiar with the TCP APIs available in Rust as well as [Tokio](https://github.com/tokio-rs/tokio).

If you want to run it yourself:

1. First start the server
```bash
cargo run 
```
Currently the server is hardcoded to use port 3000, if it can't bind to that port it will crash.

2. On seperate terminal windows (or if your cool, use tmux), connect to the server using `telnet`. Start atleast two clients
```bash
telnet localhost 3000
```

3. Chat away with yourself you weirdo!

You can end the connection with `/end`


