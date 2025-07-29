# ki

A self-explanatory key management utility to be used with [`cast`](https://github.com/foundry-rs/foundry).

ki can create and manage a set of keystores that you can easily switch between.

```bash
# Create a new key
$ ki new foo
# Note that this will error
$ cast wallet address
# Use your new key
$ . <(ki use foo)
$ cast wallet address
0x..
```
