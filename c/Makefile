CC = gcc
CFLAGS = -Wall -Wextra
DEBUG_FLAGS = -g -DDEBUG
BIN_DIR = bin

# Detect operating system
OS := $(shell uname -s)

# List of source files
SRCS = $(filter-out container.c, $(wildcard *.c))


# Exclude container.c if not Linux
ifeq ($(OS), Linux)
    SRCS += container.c
endif


# Generate binary names by replacing .c with nothing and prefixing with bin/
BINS = $(SRCS:%.c=$(BIN_DIR)/%)

# Default target is now debug
all: debug

# Debug target
debug: CFLAGS += $(DEBUG_FLAGS)
debug: $(BIN_DIR) $(BINS)

# Release target
release: CFLAGS += -O2
release: $(BIN_DIR) $(BINS)

# Create bin directory if it doesn't exist
$(BIN_DIR):
	mkdir -p $(BIN_DIR)

# Rule to build binaries
$(BIN_DIR)/%: %.c
	$(CC) $(CFLAGS) $< -o $@

# Clean target
clean:
	rm -rf $(BIN_DIR)

.PHONY: all clean debug release
