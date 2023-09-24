# Arcanum
## A framework for making mvc web apps with rust as the backend

### What is Arcanum?
Arcanum is a framework for making web apps with rust as the backend. It is designed to be easy to use and easy to learn. It is also designed to be fast and secure. It is currently in development and is not ready for production use.

### How do I use Arcanum?
You first have to make a rust project with
```bash
cargo init
```
Then you have to add Arcanum as a dependency using carg
```bash
cargo add arcanum
```
Then you have to add the following code to your main.rs file
```rust
let mut ac = ApplicationController::basic("127.0.0.1", 8080 ,vec![
    (Method::GET, "/", Route::html(|_| {"<h1>Hello /</h1>".to_string()})),
]);
ac.start();
```
This on run will show a single h1 and will be hosted on 127.0.0.1:8080

### Examples???
look in the `examples` folder for examples, some are not really done yet or outdated... but they are there


### MVC???
Why mvc, in the modern world we've sort of gotten away from mvc, but I think it is still a good way to organize code. It is also a good way to organize web apps. It makes it easy to see where certain pieces of code are.

Now what of mvc currently works:
- [ ] Models
- [x] Views, in the form of templates using a django esque syntax
- [x] Controllers, in the form of routes that can be used to render templates or return json or raw whatever floats your boat

### What is planned for Arcanum?
- [x] Basic routing
- [x] Basic templating
- [ ] Basic database support
- [ ] Basic authentication
- [ ] Basic authorization
- [ ] Basic logging
- [ ] Basic documentation
- [ ] Basic file serving

### Why the name Arcanum?
Honestly chatGPT. I was trying to think of a name and I was talking to chatGPT and it suggested Arcanum. I liked it so I went with it.