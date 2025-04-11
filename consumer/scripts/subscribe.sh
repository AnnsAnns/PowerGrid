#!/usr/bin/env bash

MQTT_BROKER="mqttbroker"

docker exec -it $MQTT_BROKER sh -c "mosquitto_sub -v -t '#'"
