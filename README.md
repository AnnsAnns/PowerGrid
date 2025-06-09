# PowerGrid

http://127.0.0.1:1880 for config
http://127.0.0.1:1880/ui for dashboard

## Building the Docker Image

There are two ways to build the docker image:

`make build_native` - Builds directly on the host machine (Requires Rust and Linux)
`make build_docker` - Builds inside a Docker container

The standard is build_native since it drastically reduces build time, but if you want to build on a different platform, use build_docker.

