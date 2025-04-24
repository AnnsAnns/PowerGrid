#!/usr/bin/env bash

echo "Stopping containers..."
docker stop dashboard
for i in $(docker ps -q --filter "name=chaos_sensor"); do
    docker stop $i
done
for i in $(docker ps -q --filter "name=sink"); do
    docker stop $i
done
docker stop tick_gen
docker stop mqttbroker

echo -e "\nRemoving containers and network...\n"
docker rm dashboard
for i in $(docker ps -aq --filter "name=chaos_sensor"); do
    docker rm $i
done
for i in $(docker ps -aq --filter "name=sink"); do
    docker rm $i
done
docker rm tick_gen
docker rm mqttbroker
docker network rm cps-net
