import paho.mqtt.client as mqtt
import logging
import sys


class MQTTWrapper:
    def __init__(self, broker_ip, broker_port, name='MQTTWrapper',
                 subscriptions=None, on_message_callback=None,
                 log_level=logging.INFO):
        self.broker_ip = broker_ip
        self.broker_port = broker_port
        self.name = name
        self.subscriptions = subscriptions
        self.on_message_callback = on_message_callback
        self.log_level = log_level

        # Configure logging
        self.log = logging.getLogger(self.name)
        self.log.setLevel(self.log_level)
        # create console handler with a higher log level
        ch = logging.StreamHandler(sys.stderr)
        ch.setLevel(self.log_level)
        self.log.addHandler(ch)

        self.client = mqtt.Client(mqtt.CallbackAPIVersion.VERSION1, self.name)
        self.client.on_connect = self.on_connect
        self.client.on_message = self.on_message

        self.client.connect(self.broker_ip, self.broker_port, 60)
        self.client.loop_start()

    def publish(self, topic, message):
        self.log.debug('publish ' + str(message) + ' to topic ' + topic)
        self.client.publish(topic, str(message))

    def subscribe(self, topic):
        self.log.debug('subscribe to  ' + topic)
        self.client.subscribe(topic)

    def subscribe_with_callback(self, sub, callback):
        self.client.message_callback_add(sub, callback)

    # The callback for when the client receives a CONNACK response from the server.
    def on_connect(self, client, userdata, flags, rc):
        self.log.info("Connected to " + self.broker_ip + ":" + str(self.broker_port) + " with result code " + str(rc))

        # Subscribing in on_connect() means that if we lose the connection and
        # reconnect then subscriptions will be renewed.
        if self.subscriptions is not None:
            for sub in self.subscriptions:
                self.log.info('subscribe to ' + sub)
                self.client.subscribe(sub)

    def on_message(self, client, userdata, msg):
        if self.on_message_callback is not None:
            self.on_message_callback(userdata, msg)
        else:
            self.log.debug(str(userdata) + ' - ' + msg.topic + ':'
                + msg.payload.decode("utf-8"))

    def stop(self):
        self.client.loop_stop()
