## Quick Start
Start db

```
docker run -d -p 5432:5432 \
  -e POSTGRES_USER=loco \
  -e POSTGRES_DB=needs_development \
  -e POSTGRES_PASSWORD="loco" \
  postgres:15.3-alpine
```

Start app
```sh
cargo loco start
```

## Deploy
```
docker buildx build --platform linux/amd64 --output ./TMP .
scp TMP/usr/app/needs-cli  root@v2202204174441188151.happysrv.de:/home/server/needs/
scp -r config  root@v2202204174441188151.happysrv.de:/home/server/needs/
scp -r assets  root@v2202204174441188151.happysrv.de:/home/server/needs/
scp -r dockerfile  root@v2202204174441188151.happysrv.de:/home/server/needs/
scp -r docker-compose.yaml  root@v2202204174441188151.happysrv.de:/home/server/needs/

```

Then on the server
```
docker compose build
docker compose down
docker compose up -d
```

## Features so far
* list of needs at http://localhost:5150/needs
* create, edit, update and delete needs
* using htmx
* authentication
  * you always work with your own user id
  * login page
  * pass BEARER token automatically via HTMX extension
    * https://htmx.org/examples/async-auth/
* deploy
  * anywhere in the web
  * dockerize to make sure it runs always
  * CI
  * progressive webapp
* future fancy features
  * Nicer login page
  * Feelings
  * see other users' needs
  * show when last updated
  * user creation interface
  * Install instructions
  * Introduction page / doc / tutorial
  * Make event-sourced
  * Add tests (FE and BE)
