#!/bin/bash

cargo build --release || exit 1

#Remove old data
rm ./sqlite.db*

#Start smart-house (rest-api) server 
RUST_LOG=info HOST=0.0.0.0 PORT=8089 DB_URL=sqlite://sqlite.db NAME="Sweet Home" nohup ./target/release/restapi_smarthouse &
let pidsrv=$!
sleep 3
echo "Server stated with pid:${pidsrv}"

#Generate testing data
URL=http://0.0.0.0:8089/ ./target/release/client_cli room-add room1
URL=http://0.0.0.0:8089/ ./target/release/client_cli room-add room2
URL=http://0.0.0.0:8089/ ./target/release/client_cli room-add room3

URL=http://0.0.0.0:8089/ ./target/release/client_cli device-add 1 socket1 socket
URL=http://0.0.0.0:8089/ ./target/release/client_cli device-add 2 socket1 socket
URL=http://0.0.0.0:8089/ ./target/release/client_cli device-add 3 socket1 socket
URL=http://0.0.0.0:8089/ ./target/release/client_cli device-add 3 thermo1 thermometer

#Start generator for socket1 in room2
URL=http://0.0.0.0:8089/ ROOM=room2 DEVICE=socket1 nohup ./target/release/gen_socket &
let pidgen1=$!
echo "Generator for socket1 in room2 started with pid:${pidgen1}"

URL=http://0.0.0.0:8089/ ROOM=room3 DEVICE=thermo1 nohup ./target/release/gen_thermometer &
let pidgen2=$!
echo "Generator for thermo1 in room3 started with pid:${pidgen2}"


./target/release/client_gui

kill -15 ${pidgen2}
kill -15 ${pidgen1}
kill -15 ${pidsrv}

#Remove old data
rm ./sqlite.db*

