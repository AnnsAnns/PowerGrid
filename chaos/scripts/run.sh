#!/usr/bin/env bash
echo "Create network..."
docker network create cps-net

echo "Starting MQTT Broker..."
docker run -d -p 127.0.0.1:8883:1883 --net=cps-net --name mqttbroker \
  eclipse-mosquitto:1.6.13

echo "Starting Tick Generator..."
docker run -d --net=cps-net --name tick_gen tick_gen:0.1

echo "Starting dashboard..."
docker run -d -p 127.0.0.1:1880:1880 -v node_red_data:/data --net=cps-net --name dashboard dashboard:0.1
echo "Starting Chaos Sensor..."
docker run -d --net=cps-net --name chaos_sensor chaos_sensor:0.1

echo "Starting Sink..."
docker run -d --net=cps-net --name sink sink:0.1