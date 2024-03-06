# Compiler and linker definitions
NASM := nasm
LD := ld

# Directories
SRC_DIR := .
BIN_DIR := ./bin

# Find all .s files in the source directory
SOURCES := $(wildcard $(SRC_DIR)/*.s)
# Derive object files from sources
OBJS := $(SOURCES:.s=.o)
# Derive target executables from sources
TARGETS := $(patsubst $(SRC_DIR)/%.s,$(BIN_DIR)/%,$(SOURCES))

# Default target: build all executables
.PHONY: all
all: $(TARGETS)

# Rule to build executables: depends on object files
$(BIN_DIR)/%: $(SRC_DIR)/%.o | $(BIN_DIR)
	$(LD) -o $@ $<

# Rule to compile assembly to object files
$(SRC_DIR)/%.o: $(SRC_DIR)/%.s
	$(NASM) -f elf64 -o $@ $<

# Ensure binary directory exists
$(BIN_DIR):
	mkdir -p $@

# Clean up build artifacts
.PHONY: clean
clean:
	rm -rf $(BIN_DIR) $(OBJS)
