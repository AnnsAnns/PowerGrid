import os
import sys
import json
import time
import csv
import random
import logging
from datetime import datetime, timedelta, time
from random import seed
from random import randint
from mqtt.mqtt_wrapper import MQTTWrapper

NAME = os.environ.get('EC_NAME')
SLP_FILE = os.environ.get('EC_SLP_FILE')
SLP_IDX = int(os.environ.get('EC_SLP_IDX'))
POPULATION_FACTOR = int(os.environ.get('EC_POPULATION_FACTOR')) 
MQTT_BASE_TOPIC = os.environ.get('EC_MQTT_TOPIC')
TICK_TOPIC = "tickgen/tick"
slp_data = None
scale_factor = 1.0

def get_slp(file, idx):
    with open(file, mode='r') as inp:
        reader = csv.reader(inp)
        next(reader)
        dict_from_csv = {rows[0]:float(rows[idx]) for rows in reader}
    return dict_from_csv

def on_message_scale(client, userdata, msg):
    global scale_factor
    scale_factor = float(msg.payload.decode("utf-8"))

def on_message_tick(client, userdata, msg):
    global MQTT_BASE_TOPIC
    global slp_data


    ts_iso = msg.payload.decode("utf-8")
    ts = datetime.fromisoformat(ts_iso)
    idx = time(hour=ts.hour, minute=(int(ts.minute/15)*15), second=0, microsecond=0).isoformat()

    offset = random.uniform(-0.05,0.05)
    # SLP Reference 1000kWh/a. Assumption of 2000kWh/a per person => * 2
    value = slp_data[idx] * 2 * POPULATION_FACTOR * (1 + offset) * scale_factor
    data = {"payload": value, "timestamp": ts_iso}
    client.publish(MQTT_BASE_TOPIC+"/demand", json.dumps(data))

def main():
    print(MQTT_BASE_TOPIC)

    global slp_data
    slp_data = get_slp(SLP_FILE, SLP_IDX)

    mqtt = MQTTWrapper('mqttbroker', 1883, name=NAME)
    mqtt.subscribe(TICK_TOPIC)
    mqtt.subscribe_with_callback(TICK_TOPIC, on_message_tick)
    mqtt.subscribe(MQTT_BASE_TOPIC + "/scale")
    mqtt.subscribe_with_callback(MQTT_BASE_TOPIC + "/scale", on_message_scale)

    try:
        mqtt.loop_forever()
    except(KeyboardInterrupt, SystemExit):
        mqtt.stop()
        sys.exit("KeyboardInterrupt -- shutdown gracefully.")

if __name__ == '__main__':
    main()
