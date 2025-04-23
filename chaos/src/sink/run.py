import sys
import json
from mqtt.mqtt_wrapper import MQTTWrapper
from collections import deque

# MQTT topic for publishing mean data
MEAN_DATA_TOPIC = "sink/1/data"

# MQTT topic for receiving tick messages
CHAOS_DATA_TOPIC = "chaossensor/1/data"
buffer = deque(maxlen=42)

def on_message_sensor(client, userdata, msg):
    global MEAN_DATA_TOPIC
    
    data = json.loads(msg.payload.decode("utf-8"))
    print(f"Received data: {data}")
    buffer.append(data['payload'])
    mean = sum(buffer) / len(buffer)
    
    # Timestamp vom Tick-Generator verwenden
    new_msg = {"payload": mean, "timestamp": data['timestamp']}
    client.publish(MEAN_DATA_TOPIC, json.dumps(new_msg))

def main():    
    # Initialize the MQTT client and connect to the broker
    mqtt = MQTTWrapper('mqttbroker', 1883, name='sink_1')
    
    # Subscribe to the tick topic
    mqtt.subscribe(CHAOS_DATA_TOPIC)
    # Subscribe with a callback function to handle incoming tick messages
    mqtt.subscribe_with_callback(CHAOS_DATA_TOPIC, on_message_sensor)
    
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
