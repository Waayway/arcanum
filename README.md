# Arcanum
**THIS PROJECT IS STILL IN DEVELOPMENT SO IT ISN'T DONE YET AND THINGS WILL CHANGE**


This is a side project i've started to create a simple framework for myself.

TODO list:

-   [x] Basic functionality (Routing, files)
-   [x] Templating
-   [ ] Models (DB)
-   [ ] docs


## How to use
to use this library add
```toml
[dependencies]
arcanum = "0.1"
serde = { version = "1.0", features = ["derive"] }
```
to the cargo.toml file.

to instantiate and run the basic web server is:

```rust
let mut server = WebServer::new("127.0.0.1", 7878);
server.run();
```
where the "127.0.0.1" is the address and 7878 is the port of the server.

to add a simple route use 
```rust
server.add_simple_route("/", handle_main_route);
```

Where `handle_main_route` is a function defined in the main.rs file

to define everything needed for handle_main_route like
```rust
#[derive(Serialize)]
struct HomepageContext {
    title: String,
}

fn handle_main_route(_req: Request, _res: &mut Response) -> ReturnData {
    let context = HomepageContext {
        title: "Hello, world!".to_string(),
    };
    let template = Template::render_template("views/index.html", context);
    return ReturnData::Text(template);
}
```
the HomepageContext gets used by Template to see what to insert into the template, you could add 90 other variables to it if it would be deemed necessary.
the `views/index.html` content is like so
```html
<!DOCTYPE html>
<html lang="en">
    <head>
        <title>Home page</title>
    </head>
    <body>
        <h1>{ title }, Title from rust content Object</h1>
    </body>
</html>
```

to add a route with params like an id or something similar we would use
```rust
server.add_route_with_params("/id/:id", handle_id_route);
```
in this case the /id route would still match this and no param would get passed (still working on that)

in this case the handle_id_route is handled like so:
```rust 

#[derive(Serialize)]
struct IdPageContext {
    id: String,
}

fn handle_id_route(
    _req: Request,
    _res: &mut Response,
    params: HashMap<String, String>,
) -> ReturnData {
    let context = IdPageContext {
        id: params["id"].clone(),
    };
    let template = Template::render_template("views/id.html", context);
    return ReturnData::Text(template);
}

```
Once again we are using templates
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <title>Document</title>
</head>
<body>
    <h1>{ id } This should be the id given by the path in the browser</h1>
</body>
</html>
```
this is the template for id.

## Helper functions
`serve_static_file` serves a static file based on the path you provide this cannot be used to provide as a function in add_simple_route, so you have to wrap it. see `examples/basic/main.rs` for more info.
