# Micro HTTP Framework Library

### *written entirely in rust using nothing but standard libraries, this framework implements all core functions needed to create a simple http local server to handle http request*


#### Core Functions: 
- Hanldes tcp socket connection
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


##### Possible Improvements: 
- *Threadpooling: would cut down on the overhead of spawning/destruction of threads*
- *More freedom: currently there is quite a bit of abstraction from the user to maintain simplicity when using the framework, allowing for these constraints to be lessened could be nice as users get more advanced*

