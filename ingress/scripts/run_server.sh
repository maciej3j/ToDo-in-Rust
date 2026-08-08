#!/usr/bin/env bash

SCRIPTPATH="$(
    cd "$(dirname "$0")"
    pwd -P
)"
echo scriptpath: $SCRIPTPATH
cd $SCRIPTPATH

cd ../..
echo pwd: $(pwd)
cd frontend
npm install
npm run build
cd ../ingress
cargo clean
cargo run
