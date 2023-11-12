After running Command ->

```
cargo shuttle run --port 8000
```

```
curl -X GET http://localhost:8000/api/health
```

Zen is High Dear!
Compiler Version: testv4

```
curl -X POST -H "Content-Type: application/json" -d '{"code":"HI"}' http://localhost:8000/api/compile
```

{"output":{"Ok":"HI\nBhag yha se!!\nOk?"}}

```
curl -X POST -H "Content-Type: application/json" -d '{"username":"zen","password":"lang","email":"cpass@gmail.com"}' http://localhost:8000/api/signup
```

token

```
curl -X POST -H "Content-Type: application/json" -d '{"password":"lang","email":"cpass1@gmail.com"}' hhttp://localhost:8000/api/login
```

User not found.

```
curl -X POST -H "Content-Type: application/json" -d '{"password":"lang1","email":"cpass@gmail.com"}' http://localhost:8000/api/login
```

Invalid password.

```
curl -X POST -H "Content-Type: application/json" -d '{"password":"lang","email":"cpass@gmail.com"}' http://localhost:8000/api/login
```

token

```
curl -H "Authorization: Bearer <valid-token>" http://localhost:8000/api/private
```

You have accessed a private route

```
curl -H "Authorization: Bearer invalid" http://localhost:8000/api/private
```

Invalid token.
