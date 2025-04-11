#!/usr/bin/env bash
BASE_DIR="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )/../src/"

docker build ${BASE_DIR}gen_energy_consumer -t gen_energy_consumer:0.1
echo -e "\n\n"
