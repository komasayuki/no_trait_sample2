# No trait async

To avoid async trait restriction.

## Run

### Work as Cat
```shell
% cargo run
Hello, world!
Cat meow
```

### Work as Dog
```shell
% cargo run --features dog --no-default-features
Hello, world!
Dog meow
```