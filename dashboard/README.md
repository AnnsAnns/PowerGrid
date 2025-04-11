## CPS Dashboard
The CPS Dashboard runs on [Node-RED][1]. Node-RED is a flow-based programming tool for wiring together hardware devices, APIs and online services as part of the IoT or CPS. It provides a visual browser-based flow editor. Node-RED is build on [Node.js][2]. The [node-red-dashboard][3] module is required for visualization functionality. It offers a huge set of common building blocks, but also allows to write custom JavaScript routines.

Important URLs:
- Editor:  [http://127.0.0.1:1880/](http://127.0.0.1:1880/)
- Dashboard: [http://127.0.0.1:1880/ui](http://127.0.0.1:1880/ui)

In this project, Node-RED runs in the mode [Dockerfile which copies in local resources][4]. Thus changes to _flows_ have to be copied manually and placed next to the Dockerfile, use `docker cp` (call `man docker-cp` for help). If at least one configuration file is updated the dashboard docker image needs to be updated, i.e. build again.

To build the docker image and run it in a container call the following commands:
```bash
docker build . -t cps-dashboard:0.1
docker run -it -p 127.0.0.1:1880:1880 --name dashboard cps-dashboard:0.1
```

[1]: https://nodered.org/
[2]: https://nodejs.org/en/
[3]: https://github.com/node-red/node-red-dashboard
[4]: https://nodered.org/docs/getting-started/docker#dockerfile-which-copies-in-local-resources
