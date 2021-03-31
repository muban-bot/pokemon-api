# felix angell - truelayer coding challenge
Hi, this is my takehome test for TrueLayer. I attempted to try this in Rust to see how it would go!
I'm still new to Rust so apologies if you see any non-idiomatic Rust stuff.

## building
```bash
$ cargo build
$ cargo run
```

## what would i do differently?
I chose to write this in Rust so had to do a bit of learning of how to
get something working & how to write tests. I think the main issues I ran into were
related to trying to approach building Rust API's with the mindset of writing something in Java or Kotlin.

The general approach was to have an acceptance test to meet what the spec is asking for and then testing
each service/client using a quick test case that runs against the actual endpoints.

Ideally this would be mocked, but I couldn't find a good mocking library to commit to in time! i.e. not
opening ports/connections in test cases as this doesn't scale well in practice e.g. on a ci/cd pipeline.

Also getting these to work together with the existing http server lib. was making things more convoluted than necessary
so to not over-do things I tried to keep it simple instead.

Another problem is error handling, I'm not entirely sure of the idioms for handling errors nicely in Rust
using all the libraries and new syntactic sugar.

Not only this but I'm a big fan of static  code generation, so it would be cool to generate from a schema instead and keep things consistent
but for this case I chose to partially implement an api for what was necessary.

Lastly, due to the apis being rate limited there is definitely room for caching in place with an LRU cache or something
to store prior pokemon searches as the data is pretty much static.

## notes
```
// the input/endpoint thing
http://localhost:5000/pokemon/charizard

// output response body
{
	"name": "charizard",
	"description": "this is the description of the charizard pokemon but shakespeare"
}
```

### todo stuff:

1. automated tests
2. structure project as if its a production thing
3. apis: pokeapi.co - funtranslations.com/api/shakespeare

readme:

1. how to run the thing
2. what im not happy with if i had more time
3. add a dockerfile
4. git history
