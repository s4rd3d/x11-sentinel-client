# Common directories and paths
TOP_DIR := $(dir $(firstword $(MAKEFILE_LIST)))
ABS_DIR := $(abspath $(TOP_DIR))

# Build directory
BUILD_DIR := $(ABS_DIR)/target

#-------------------------------------------------------------------------------
# Targets
#-------------------------------------------------------------------------------

# Install dependencies
.PHONY: install-deps
install-deps:
	cargo fetch

# Compile and optimize the application
.PHONY: compile
compile:
	cargo build --release

# Run the built binary
.PHONY: run
run:
	cargo run

# Cleanup targets
.PHONY: mrproper
mrproper:
	$(RM) -r $(BUILD_DIR)
