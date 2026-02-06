## docker compose
because the servers are in the same network we dont use localhost, we use the service name

## commands

docker compose build

docker compose up

docker compose down

front will make calls to
fetch('/api/rx/random-endpoint')
fetch('/api/tx/random-endpoint')
