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


## Routes

| Route              | Type | Headers                                         | Content                                      | Description                                         |
|---------------------|------|-------------------------------------------------|----------------------------------------------|-----------------------------------------------------|
| `/api/health`       | GET  | None                                            | None                                         | Check the health of the system.                      |
| `/api/compile`      | POST | Content-Type: application/json                  | {"code": "String"}                           | Compile the provided code.                          |
| `/api/signup`       | POST | Content-Type: application/json                  | {"username": "String", "password": "String", "email": "String"} | Register a new user.                               |
| `/api/login`        | POST | Content-Type: application/json                  | {"email": "String", "password": "String"}   | Log in with user credentials.                       |
| `/api/private`      | GET  | Authorization: Bearer `<valid-token>`           | None                                         | Access a private route with a valid token.          |


## Deployed Using

[shuttle.rs](https://console.shuttle.rs)
