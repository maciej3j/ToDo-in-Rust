#!/usr/bin/env bash

SCRIPT_PATH="$(
    cd "$(dirname "$0")"
    pwd -P
)"
cd $SCRIPT_PATH
cd ..

framework=${1:-actix}
case $framework in
actix)
    echo "Building and running actix server..."
    cargo build -p to-do-actix-server
    cargo run -p to-do-actix-server &
    PID=$!
    ;;
axum)
    echo "Building and running axum server..."
    cargo build -p to-do-axum-server
    cargo run -p to-do-axum-server &
    PID=$!
    ;;
*)
    echo "Unknown framework: $framework!"
    exit 1
    ;;

esac
sleep 1

rm tasks.json
rm output.txt
cat <<EOF >tasks.json
{}
EOF
echo "Server started with PID: $PID"

curl -X POST http://127.0.0.1:8000/api/v1/create \
    -H "Content-Type: application/json" \
    -d '{"title": "writing", "status": "Pending"}' >>output.txt
echo "" >>output.txt
curl -X POST http://127.0.0.1:8000/api/v1/create \
    -H "Content-Type: application/json" \
    -d '{"title": "coding", "status": "Pending"}' >>output.txt
echo "" >>output.txt
curl -X DELETE http://127.0.0.1:8000/api/v1/delete/coding >>output.txt

echo "" >>output.txt
curl -X PATCH http://127.0.0.1:8000/api/v1/patch \
    -H "Content-Type: application/json" \
    -H "token: some_token" \
    -d '{"title": "writing", "status": "Done"}' >>output.txt

kill $PID
