import os
import sys
import json
from mqtt.mqtt_wrapper import MQTTWrapper
from collections import deque

# Buffer for storing the last 42 values
buffer = deque(maxlen=42)
id = os.getenv("ID", "1")

def on_message_sensor(client, userdata, msg):    
    data = json.loads(msg.payload.decode("utf-8"))
    buffer.append(data['payload'])
    mean = sum(buffer) / len(buffer)
    
    # Timestamp vom Tick-Generator verwenden
    new_msg = {"payload": mean, "timestamp": data['timestamp']}
    client.publish("sink/" + str(id) + "/data", json.dumps(new_msg))

def main():
    # Initialize the MQTT client and connect to the broker
    mqtt = MQTTWrapper('mqttbroker', 1883, name='sink_' + str(id))
    
    # Subscribe to the tick topic
    mqtt.subscribe("chaossensor/" + str(id) + "/data")
    # Subscribe with a callback function to handle incoming tick messages
    mqtt.subscribe_with_callback("chaossensor/" + str(id) + "/data", on_message_sensor)
    
    try:
        # Start the MQTT loop to process incoming and outgoing messages
        mqtt.loop_forever()
    except (KeyboardInterrupt, SystemExit):
        # Gracefully stop the MQTT client and exit the program on interrupt
        mqtt.stop()
        sys.exit("KeyboardInterrupt -- shutdown gracefully.")

if __name__ == '__main__':
    # Entry point for the script
    main()
