## About
Bearer authentication with actix-web

### Setup
```
# or change hello to whatever value, but make sure to use that value in the next step
echo 'BEARER=hello' > .env
cargo run
```

## How to confirm it's working
```bash
curl -v localhost:8080 -H "Authorization: Bearer hello"
```
should return
`Test` (with status code 200)


```bash
curl -v localhost:8080 -H "Authorization: Bearer somethingelse"
```
should return
`Oops not allowed` (with status code 401)
