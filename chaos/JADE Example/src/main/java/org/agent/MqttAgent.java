package org.agent;

import jade.core.AID;
import jade.core.Agent;
import jade.core.behaviours.CyclicBehaviour;
import jade.core.behaviours.OneShotBehaviour;
import org.eclipse.paho.client.mqttv3.IMqttMessageListener;
import org.eclipse.paho.client.mqttv3.MqttClient;
import org.eclipse.paho.client.mqttv3.MqttException;
import org.eclipse.paho.client.mqttv3.MqttMessage;
import org.eclipse.paho.client.mqttv3.persist.MemoryPersistence;


import java.util.UUID;
import org.json.JSONObject;

public class MqttAgent extends Agent {
    private String broker = "tcp://127.0.0.1:8883"; // MQTT Broker-Adresse
    private String clientId = UUID.randomUUID().toString();
    private MqttClient mqttClient;
    private static String BASE_TOPIC = "JadeExample/";


    //Moving Average
    MovingAverage myAverage = new MovingAverage(10);

    protected void setup() {
        System.out.println("Agent " + getLocalName() + " is ready.");
        try {
            mqttClient = new MqttClient(broker, clientId, new MemoryPersistence());
            mqttClient.connect();

            addBehaviour(new SubscribeBehaviour());
            addBehaviour(new PublishBehaviour());
        } catch (MqttException e) {
            e.printStackTrace();
        }
    }
    protected void takeDown() {
        try {
            mqttClient.disconnect();
        } catch (MqttException e) {
            e.printStackTrace();
        }
    }
    private class SubscribeBehaviour extends CyclicBehaviour {
        public void action() {
            try {
                mqttClient.subscribe("chaossensor/1/data", new IMqttMessageListener() {
                    public void messageArrived(String topic, MqttMessage message) throws Exception {
                        String msg = new String(message.getPayload());
                        System.out.println("Received message on topic " + topic + ": " + msg);

                        JSONObject myData = new JSONObject(msg);

                        myAverage.addValue(myData.getDouble("payload"));
                        Double newMA = myAverage.getMovingAverage();

                        MqttMessage newMessage = new MqttMessage(Double.toString(newMA).getBytes());

                        try {
                            mqttClient.publish(BASE_TOPIC + clientId, newMessage);
                        } catch (MqttException e) {
                            e.printStackTrace();
                        }
                    }
                });
            } catch (MqttException e) {
                e.printStackTrace();
            }
        }
    }

    private class PublishBehaviour extends OneShotBehaviour {
        public void action() {
            String content = "Hello from JADE MQTT Agent";
            MqttMessage message = new MqttMessage(content.getBytes());
            try {
                mqttClient.publish(BASE_TOPIC + clientId, message);
                System.out.println("Published message: " + content);
            } catch (MqttException e) {
                e.printStackTrace();
            }
        }
    }


}
