package org.container;
import jade.Boot;
import jade.core.Profile;
import jade.core.ProfileImpl;
import jade.wrapper.AgentContainer;
import jade.wrapper.AgentController;
import jade.wrapper.StaleProxyException;
import org.agent.MqttAgent;

public class MainContainer {
    public static void main(String[] args) {
        jade.core.Runtime runtime = jade.core.Runtime.instance();
        Profile profile = new ProfileImpl();
        profile.setParameter(Profile.GUI, "true");
        AgentContainer container = runtime.createMainContainer(profile);

        // start the JADE RMA GUI
        String[] rmaArguments = new String[] { "-gui" };
        Boot.main(rmaArguments);

        int numberOfAgents = 5; // number of agents

        for (int i = 0; i < numberOfAgents; i++) {
            try {
                AgentController agentController = container.createNewAgent("agent" + i, MqttAgent.class.getName(), null);
                agentController.start();
            } catch (StaleProxyException e) {
                e.printStackTrace();
            }
        }
    }

}
