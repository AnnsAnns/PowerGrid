.PHONY: all
all: build_docker

.PHONY: change_to_docker_build
change_to_docker_build:
	@echo "Changing to docker build ..."
	@cp Dockerfile.build_docker Dockerfile

.PHONY: change_to_native_build
change_to_native_build:
	@echo "Changing to native build ..."
	@cp Dockerfile.build_native Dockerfile

.PHONY: build_native
build_native: change_to_native_build
	@echo "Building directly on the host system ..."
	@cargo build --release
	mkdir -p docker_data
	@docker build . -t powergrid_base
	@COMPOSE_BAKE=true docker-compose up --build

.PHONY: build_docker
build_docker: change_to_docker_build
	@echo "Building inside a Docker container ..."
	mkdir -p docker_data
	@docker-compose up --build

start_turbine:
	@docker-compose up --build turbine

.PHONY: clean
clean: remove_artifacts remove_docker_data

.PHONY: remove_artifacts
remove_artifacts:
	@echo "Removing artifacts ..."
	@rm -rf target logs

.PHONY: remove_docker_data
remove_docker_data:
	@echo "Removing docker_data directory, requires sudo ..."
	@sudo rm -rf .docker_data

.PHONY: rebuild
rebuild:
	@docker build . -t base