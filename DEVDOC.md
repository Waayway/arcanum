# DEVDOCS
### These are some simple docs so i can explain concepts to myself.


## Routing
Routing should be handle by add_route function in main.rs,

Basic Example is:
```rust
server.add_route("/", handle_main_route);
```

One liner could be done like: 
```rust
server.add_route("/img", |_req: Request, res: &mut Response| serve_image("public/user.png", res));
```

also wildcards should be added like
```rust
server.add_route("/public/*", serve_static_route);
```
This should only serve a single directory of files.
to also serve subfolders it should be ** instead of * like
```rust
server.add_route("/public/**", serve_static_route);
```
This will serve all files in the public folder.


to get data from the path it should be a : identifier and a name behind it like:
```rust
server.add_route("/:id", handle_id_route);
```