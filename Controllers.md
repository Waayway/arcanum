# Controllers
Since there's not really inherintance (its a good thing). we have to use a different way to make controllers.

## Creating a controller
```rust

struct Controller {
    // ...
    bs: BaseController // Some sort of base controller which we can implement the db and templating
} 

impl Routing for Controller { // This is the important part which allows us to use the router to route to this controller..... but idk if this is the best way to do it
    fn routes(&self) -> Vec<Route> {
        vec![
            Route::new("/path", "GET", Self::function),
        ]
    }
}
impl Controller {
    pub fn function(&self, /* Still not final but idk yet */) -> String {
        // ...
        self.bs.template("template_name", data /* I have no idea what i want here but probably this....*\);
    }
}
```