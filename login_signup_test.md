After running Command ->

```bash
cargo shuttle run --port 8000
```

Run the command in other terminal to run httpc-tests

```
cargo watch -q -c -w examples/ -x 'run --example quick_dev'
```

### MANUAL CURL TESTS COMMANDS

```bash
curl -X GET http://localhost:8000/api/health
```

Zen is High Dear!
Compiler Version: testv4

```bash
curl -X POST -H "Content-Type: application/json" -d '{"code":"HI"}' http://localhost:8000/api/compile
```

{"output":{"Ok":"HI\nBhag yha se!!\nOk?"}}

```bash
curl -X POST -H "Content-Type: application/json" -d '{"name":"ME","username":"zen","password":"lang","email":"cpass@gmail.com"}' http://localhost:8000/api/signup
```

token

```bash
curl -X POST -H "Content-Type: application/json" -d '{"password":"lang","email":"cpass1@gmail.com"}' http://localhost:8000/api/login
```

User not found.

```bash
curl -X POST -H "Content-Type: application/json" -d '{"password":"lang1","email":"cpass@gmail.com"}' http://localhost:8000/api/login
```

Invalid password.

```bash
curl -X POST -H "Content-Type: application/json" -d '{"password":"lang","email":"cpass@gmail.com"}' http://localhost:8000/api/login
```

token

```bash
curl -H "Authorization: Bearer <valid-token>" http://localhost:8000/api/private
```

You have accessed a private route

```bash
curl -H "Authorization: Bearer <invalid>" http://localhost:8000/api/private
```

Invalid token.
