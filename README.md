# PyPathfinding

Using this as an excuse to learn more about python bindings for Rust.

Providing utilties and access to the [Rust Pathfinding](https://github.com/evenfurther/pathfinding) library through basic graph classes.

## To Build and Test

Assuming you have cargo and pipenv installed, then from the repo root:

```
# Setup the environment
$ pipenv install

# Build
$ pipenv run maturin develop

# Run tests
$ pipenv run pytest test/

# Run benchmarks
$ pipenv run pytest benches/
```
