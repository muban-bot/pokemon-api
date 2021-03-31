# felix angell

## building
```bash
$ cargo build
$ cargo run
```

This should run on port 5000 so you can hit requests to localhost:5000 via. paw or some api client.

## notes
I chose to write this in Rust so had to do a bit of learning of how to
get something working & how to write tests. The main issues I ran into were
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
