#!/usr/bin/env bash

echo "Stopping containers..."
docker stop dashboard
docker stop chaos_sensor
docker stop sink
docker stop tick_gen
docker stop mqttbroker

echo -e "\nRemoving containers and network...\n"
docker rm dashboard
docker rm chaos_sensor
docker rm sink
docker rm tick_gen
docker rm mqttbroker
docker network rm cps-net
