#!/bin/bash


function start_rust_server(){
   ./app/chord_rust &
   rust_server_pid=$!
   
}

function run_flutter_app() {
    ./app/bundle/chord_flutter &
    flutter_app_pid=$!
}

function cleanup() {
  # Check if Rust server is running and kill it
  if [[ -n "$rust_server_pid" ]]; then
    kill -TERM "$rust_server_pid"
    wait "$rust_server_pid"  
  fi

  # Check if Flutter app is running and kill it
  if [[ -n "$flutter_app_pid" ]]; then
    kill -TERM "$flutter_app_pid"
    wait "$flutter_app_pid" 
  fi

  echo "Exiting..." 
  exit 0
}

trap cleanup INT

function main() {
    start_rust_server
    run_flutter_app
    wait 
}

main
