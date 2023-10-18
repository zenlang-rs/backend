# zen-server

### To Run the zen-server


```bash
cargo install cargo-shuttle
cargo shuttle run
```

### To deploy

```bash
cargo shuttle deploy
```

### To clean cache


```bash
cargo shuttle clean
```

### Deploy workaround
Currently, if "zen" packages updates, then we need to change Cargo.toml to pin to the latest version and then redeploy! [Searching for workaround]

## Deployed Using

[shuttle.rs](https://console.shuttle.rs)
