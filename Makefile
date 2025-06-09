.PHONY: all
all: build_docker

.PHONY: build_docker
build_docker: 
	@echo "Building inside a Docker container ..."
	@docker build . -t local/powergrid:latest
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

.PHONY: windows_build
windows_build:
	@echo "Building inside a Docker container for Windows ..." 
	@docker build . -t local/powergrid:latest
	if not exist docker_data mkdir docker_data
	@docker-compose up --build

.PHONY: windows_clean
windows_clean:
	@echo "Cleaning up Windows build artifacts ..."
	@docker compose down
	@docker rmi local/powergrid:latest
	@del /q /f .docker_data\logs\*.*