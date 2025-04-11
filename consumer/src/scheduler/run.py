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

NAME = os.environ.get('SCHED_NAME')
SCHEDULE_FILE = os.environ.get('SCHED_CONFIG_FILE')
BASE_TOPIC = os.environ.get('SCHED_BASE_TOPIC')
TICK_TOPIC = "tickgen/tick"
schedule = None
enable_managed_mode = True

def get_schedule(file):
    schedule = None
    with open(file) as f:
        schedule = json.loads(f.read())
    return schedule

def on_message_tick(client, userdata, msg):
    global schedule

    if enable_managed_mode:
        ts_iso = msg.payload.decode("utf-8")
        ts = datetime.fromisoformat(ts_iso)
        cur_hour = str(ts.hour)

        values = schedule["schedule"][cur_hour]
        for i in range(len(values)):
            prod = schedule["producer"][i]
            value = float(values[i])/100.0 * schedule["rated_power"][i]
            client.publish(prod, value)

def on_message_schedule(client, userdata, msg):
    global enable_managed_mode
    enable_managed_mode = (msg.payload.decode("utf-8").lower() == "true")

def main():

    global schedule
    schedule = get_schedule(SCHEDULE_FILE)

    mqtt = MQTTWrapper('mqttbroker', 1883, name=NAME)
    mqtt.subscribe(TICK_TOPIC)
    mqtt.subscribe_with_callback(TICK_TOPIC, on_message_tick)

    mqtt.subscribe(BASE_TOPIC + "/schedule_mode")
    mqtt.subscribe_with_callback(BASE_TOPIC + "/schedule_mode", on_message_schedule)
    
    try:
        mqtt.loop_forever()
    except(KeyboardInterrupt, SystemExit):
        mqtt.stop()
        sys.exit("KeyboardInterrupt -- shutdown gracefully.")

if __name__ == '__main__':
    main()
