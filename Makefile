# Common directories and paths
TOP_DIR := $(dir $(firstword $(MAKEFILE_LIST)))
ABS_DIR := $(abspath $(TOP_DIR))

# Build directory
BUILD_DIR := $(ABS_DIR)/target

#-------------------------------------------------------------------------------
# Targets
#-------------------------------------------------------------------------------

# Default target
.PHONY: default
default: all

# Generate docs, compile the source code and run tests
.PHONY: all
all: docs compile check test

# Cleanup targets, generate docs, compile the source code and run tests
.PHONY: everything
everything: mrproper all

# Cleanup targets
.PHONY: mrproper
mrproper:
	$(RM) -r $(BUILD_DIR)

# Cleanup built binaries
.PHONY: clean
clean:
	cargo clean

# Install dependencies
.PHONY: install-deps
install-deps:
	cargo fetch

# Upgrade dependencies
.PHONY: upgrade-deps
upgrade-deps:
	cargo update

# Compile and optimize the application
.PHONY: compile
compile:
	cargo build --release

# Generate docs
.PHONY: docs doc
docs: doc
doc:
	cargo doc

# Run the built binary
.PHONY: start run
start: run
run:
	cargo run

# Run tests
.PHONY: test
test:
	cargo test

# Check a local package and all of its dependencies for errors
.PHONY: check
check:
	cargo check
