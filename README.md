# Micro HTTP Framework Library

### *written entirely in rust using nothing but standard libraries, this framework implements all core functions needed to create a simple http local server to handle http request*


#### Core Functions: 
- Handles tcp socket connection
- Parsre raw tcp request into readable http format
- Allows mapping of user defined functions to uri's to act as middleware or reponse handlers
- Simple Encapsulated interface to configure tcp socket and framework rules
- Logging




#### Notable Features: 
- express inspired so it has simple and clean interface for user defined functions
- implementes multi threaded handling of the request
- Implements static hosting of files based off of user defined directory *(constrained to common file types currently )*
#### Novelty features: 
- state of server is represented using 3 main state types for easy understanding: 
  *UnstartedServer -> StartedServer-> StoppedServer* each only able to be created by the previous state type
- Http response type is implemneted using a  Builder Pattern with simple and concise associated functions for ease of use when construting a handler 
- Internal error system with consise naming convention


#### Usage Examples:

- Creates the *'Configureable'* which also uses the Builder Pattern is the type that holds core info needed to initialize the  *'UnstartedServer'*
``` rust
       let mut config = FrameWork::new_config(); // returns default config type
        config.set_host(HOST).set_port(PORT); 

        config.enable_debug_mode(); // enables log file to display state of server while running

        let http_server = FrameWork::new(config); // 'http_server' now holds a &'static mut UstartedServer
        //NOTE: this type needs to live for the entire duration of the program, any dropping of it will cause a fatal error
```

- Once the *'UstartedServer'* type is initalized it is where most of the interfacing with the http server will take place like assigning handler functions and middleware, both functions use the similar signatures

Creating Handler can be done using a closure 
``` rust
  let http_server = FrameWork::new(config); 
  let route_error = http.add_handler(
        "/home",
        "GET",
        |req: &mut HttpRequest, res: &mut HttpResponse| -> HandlerResult {
            res.set_status_code(200)?
                .text_to_body("hello world")?;
            return Ok(());
        },
    );
```
or using a defined function
``` rust
  fn handler_function(req: &mut HttpRequest, res: &mut HttpResponse) HandlerResult{
    res.set_status_code(200)?
    .text_to_body("hello world")?;
    return Ok(());
  }


  let http_server = Framework::new(config);
  let route_error = http.add_handler("/home", "GET", handler_function);
```
``` rust
  let http_server = FrameWork::new(config); 
  let result = http_server.add_middleware_handler("/uri", )
``` 
- Every internal action or state change is represented using a error type *'InternalServerError'*, This error is paired with internal result types to give a clean error handling and propagation of errors when any framework provided function is called. 

#### Possible Improvements: 
- *Threadpooling: would cut down on the overhead of spawning/destruction of threads*
- *More freedom: currently there is quite a bit of abstraction from the user to maintain simplicity when using the framework, allowing for these constraints to be lessened could be nice as users get more advanced*
- *Think of a name for the framework, it appears nameing a library is even more difficult than thinking of variable names*







