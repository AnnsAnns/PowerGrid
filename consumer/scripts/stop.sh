#!/usr/bin/env bash

echo "Stopping containers..."
docker kill cps_sched
docker kill cps_h0
docker kill cps_g0
docker kill cps_g1
docker kill cps_g5



echo -e "\nRemoving containers and network...\n"

docker rm cps_sched
docker rm cps_h0
docker rm cps_g0
docker rm cps_g1
docker rm cps_g5
docker rm cps_ps
