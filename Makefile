# Variables
CARGO := cargo

# Targets
.PHONY: all build run clean

all: build

build:
	$(CARGO) build

run:
	$(CARGO) run

clean:
	$(CARGO) clean
