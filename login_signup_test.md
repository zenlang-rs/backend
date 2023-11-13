After running Command ->

```bash
cargo shuttle run --port 8000
```

Run the command in other terminal to run httpc-tests

```
cargo watch -q -c -w examples/ -x 'run --example quick_dev'
```

### MANUAL CURL TESTS COMMANDS

- #### API HEALTH CHECK

```bash
curl -X GET http://localhost:8000/api/health
```

Zen is High Dear!
Compiler Version: testv4

- #### COMPILER CHECK

```bash
curl -X POST -H "Content-Type: application/json" -d '{"code":"HI"}' http://localhost:8000/api/compile
```

{"output":{"Ok":"HI\nBhag yha se!!\nOk?"}}

- #### SIGNUP

```bash
curl -X POST -H "Content-Type: application/json" -d '{"name":"<name>","username":"<user-name>","password":"<password>","email":"<email>"}' http://localhost:8000/api/signup
```

token

- #### LOGIN WITH UN-SIGNED EMAIL

```bash
curl -X POST -H "Content-Type: application/json" -d '{"password":"<password>","email":"<invalid-email>"}' http://localhost:8000/api/login
```

User not found.

- #### LOGIN WITH WRONG PASSWORD

```bash
curl -X POST -H "Content-Type: application/json" -d '{"password":"<wrong-password>","email":"<email>"}' http://localhost:8000/api/login
```

Invalid password.

- #### LOGIN WITH CORRECT CREDENTIALS

```bash
curl -X POST -H "Content-Type: application/json" -d '{"password":"<password>","email":"<email>"}' http://localhost:8000/api/login
```

token

- #### CHANGE PASSWORD WITH TOKEN

```bash
curl -X POST -H "Content-Type: application/json"  -H "Authorization: Bearer <valid-token>" -d '{"new_password":"<new-password>"}' http://localhost:8000/api/changepassword

```

Password changed successfully

- #### CHANGE PASSWORD WITH INVALID TOKEN

```bash

curl -X POST -H "Content-Type: application/json"  -H "Authorization: Bearer <invalid-token>" -d '{"new_password":"<new-password>"}' http://localhost:8000/api/changepassword
```

Invalid token.

- #### SEND EMAIL FOR RESET PASSWORD FOR UN-SIGNED EMAIL

```bash
curl -X POST  http://127.0.0.1:8000/api/send_email/<non-user-email>
```

A user with this email does not exist

- #### SEND EMAIL FOR RESET PASSWORD

```bash
curl -X POST  http://127.0.0.1:8000/api/send_email/<user-email>
```

Emails sent successfully

- #### RESET PASSWORD

```bash
curl -X POST -H "Content-Type: application/json" -d '{"verification_token":"<verification_token_from frontend url sent on mail>","new_password":"<new_password>","email":"<user-email from url or input>"}'  http://127.0.0.1:8000/api/reset
```

Password reset successfully

### NOTE : For incomplete request body , the data will not be deserialized correctly to a valid struct type.
