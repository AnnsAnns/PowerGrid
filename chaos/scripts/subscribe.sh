#!/usr/bin/env bash
docker exec -it mqttbroker sh -c "mosquitto_sub -v -t '#'"
