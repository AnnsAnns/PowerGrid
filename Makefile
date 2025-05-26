.PHONY: all
all: up

.PHONY: up
up:
	mkdir -p docker_data
	@docker-compose up --build

start_turbine:
	@docker-compose up --build turbine

.PHONY: dontcare
dontcare:
	docker-compose up

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