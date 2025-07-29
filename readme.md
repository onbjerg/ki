# ki

A self-explanatory key management utility to be used with [`cast`](https://github.com/foundry-rs/foundry).

ki can create and manage a set of keystores that you can easily switch between.

## Usage

**Create a new key**

```bash
ki new foo
```

You will notice that commands like `cast wallet address` will still complain about a missing key.

**Use the new key**

For Bash:

```bash
. <(ki use foo)
```

For fish:

```fish
ki use foo | .
```

Now `cast wallet address` will work without any additional arguments.
