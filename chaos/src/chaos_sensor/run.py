import sys
import json
import logging
from random import seed, randint
from mqtt.mqtt_wrapper import MQTTWrapper

# MQTT topic for publishing sensor data
CHAOS_DATA_TOPIC = "chaossensor/1/data"
# Seed for generating consistent random values
SEED = 42

# MQTT topic for receiving tick messages
TICK_TOPIC = "tickgen/tick"

def on_message_tick(client, userdata, msg):
    """
    Callback function that processes messages from the tick generator topic.
    It generates a random sensor value and publishes it along with the tick's timestamp.
    
    Parameters:
    client (MQTT client): The MQTT client instance
    userdata: User-defined data (not used here)
    msg (MQTTMessage): The message containing the tick timestamp
    """
    global CHAOS_DATA_TOPIC
    
    # Extract the timestamp from the tick message and decode it from UTF-8
    ts_iso = msg.payload.decode("utf-8")
    # Generate a random sensor value between 0 and 100
    value = randint(0, 100)

    # Important: Always send your data with the timestamp from the Tick message.
    # Node Red is designed for real-time or historical messages, so discrepancies 
    # in timestamps can cause errors in the display.

    data = {"payload": value, "timestamp": ts_iso}
    # Publish the data to the chaos sensor topic in JSON format
    client.publish(CHAOS_DATA_TOPIC, json.dumps(data))

def main():
    """
    Main function to initialize the MQTT client, set up subscriptions, 
    and start the message loop.
    """
    # Set the random seed for reproducibility
    seed(SEED)
    
    # Initialize the MQTT client and connect to the broker
    mqtt = MQTTWrapper('mqttbroker', 1883, name='chaossensor_1')
    
    # Subscribe to the tick topic
    mqtt.subscribe(TICK_TOPIC)
    # Subscribe with a callback function to handle incoming tick messages
    mqtt.subscribe_with_callback(TICK_TOPIC, on_message_tick)
    
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
