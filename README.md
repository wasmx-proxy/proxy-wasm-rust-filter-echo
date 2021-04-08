# proxy-wasm-filter-echo

Build with:

```
$ cargo build --target=wasm32-unknown-unknown
```

And copy the resulting Wasm bytecode to an Nginx prefix:

```
$ cp target/wasm32-unknown-unknown/debug/proxy_wasm_filter_echo.wasm /etc/nginx
```

Use it as such in `nginx.conf`:

```nginx
# nginx.conf
events {}

wasm {
    module echo /etc/nginx/proxy_wasm_filter_echo.wasm;
}

http {
    server {
        listen 9000;

        location / {
            proxy_wasm  echo;
        }
    }
}
```

