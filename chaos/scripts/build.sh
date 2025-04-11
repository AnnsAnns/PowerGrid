#!/usr/bin/env bash
BASE_DIR="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )/../src/"

docker build ${BASE_DIR}tick_gen -t tick_gen:0.1
echo -e "\n\n"

docker build ${BASE_DIR}chaos_sensor -t chaos_sensor:0.1
echo -e "\n\n"

docker build ${BASE_DIR}dashboard -t dashboard:0.1
echo -e "\n\n"
