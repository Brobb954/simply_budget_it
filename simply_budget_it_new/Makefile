# Define variables
DOCKER_COMPOSE = docker-compose
DC_BUILD = $(DOCKER_COMPOSE) build
DC_UP = $(DOCKER_COMPOSE) up
DC_DOWN = $(DOCKER_COMPOSE) down
DC_RMI = $(DOCKER_COMPOSE) down --rmi all
DC_STOP = $(DOCKER_COMPOSE) stop

# Build and start the containers
.PHONY: up
up:
	$(DC_UP) --build

# Build the containers
.PHONY: build
build:
	$(DC_BUILD)

# Start the containers without building
.PHONY: start
start:
	$(DC_UP)

# Stop the containers
.PHONY: stop
stop:
	$(DC_STOP)

# Remove containers and images
.PHONY: clean
clean:
	$(DC_DOWN) --rmi all

# Remove containers, networks, volumes, and images
.PHONY: down
down:
	$(DC_DOWN)

# Logs
.PHONY: logs
logs:
	$(DOCKER_COMPOSE) logs -f


