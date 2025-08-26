# Welcome to Loco :train:

[Loco](https://loco.rs) is a web and API framework running on Rust.

This is the **SaaS starter** which includes a `User` model and authentication based on JWT.
It also include configuration sections that help you pick either a frontend or a server-side template set up for your fullstack server.


## Quick Start

```sh
cargo loco start
```

```sh
$ cargo loco start
Finished dev [unoptimized + debuginfo] target(s) in 21.63s
    Running `target/debug/myapp start`

    :
    :
    :

controller/app_routes.rs:203: [Middleware] Adding log trace id

                      ▄     ▀
                                 ▀  ▄
                  ▄       ▀     ▄  ▄ ▄▀
                                    ▄ ▀▄▄
                        ▄     ▀    ▀  ▀▄▀█▄
                                          ▀█▄
▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄   ▄▄▄▄▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄▄▄ ▀▀█
 ██████  █████   ███ █████   ███ █████   ███ ▀█
 ██████  █████   ███ █████   ▀▀▀ █████   ███ ▄█▄
 ██████  █████   ███ █████       █████   ███ ████▄
 ██████  █████   ███ █████   ▄▄▄ █████   ███ █████
 ██████  █████   ███  ████   ███ █████   ███ ████▀
   ▀▀▀██▄ ▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀ ██▀
       ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
                https://loco.rs

environment: development
   database: automigrate
     logger: debug
compilation: debug
      modes: server

listening on http://localhost:5150
```

## Start postgresql
```
docker run -d -p 5432:5432 \
  -e POSTGRES_USER=loco \
  -e POSTGRES_DB=needs_development \
  -e POSTGRES_PASSWORD="loco" \
  postgres:15.3-alpine
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
* future fancy features
  * make it look ok
  * improve login UX
    * one wrapper page that can be accessed w/o login
    * if logged out show login
    * otherwise show needs list
  * deploy
    * CI
    * domain w/o port
  * see other users' needs
  * show when last updated
