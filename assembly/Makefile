# Compiler and linker definitions
NASM := nasm
LD := ld
CC := gcc

CFLAGS := -O0 -g

# Directories
SRC_DIR := .
BIN_DIR := ./bin

# Find all .s files in the source directory
SOURCES := $(wildcard $(SRC_DIR)/*.s)
# Derive object files from sources
OBJS := $(SOURCES:.s=.o)
# Derive target executables from sources
TARGETS := hello sum loop

# Default target: build all executables
.PHONY: all
all: $(BIN_DIR)/hello $(BIN_DIR)/sum $(BIN_DIR)/total $(BIN_DIR)/loop

# Rule to build executables: depends on object files
$(BIN_DIR)/hello: hello.o | $(BIN_DIR)
	$(LD) -o $@ $<

$(BIN_DIR)/loop: loop.o | $(BIN_DIR)
	$(CC) -no-pie -nostartfiles loop.o -o $(BIN_DIR)/loop

$(BIN_DIR)/sum: asm_func.o sum.c | $(BIN_DIR)
	$(CC) -o $@ sum.c asm_func.o

$(BIN_DIR)/total: asm_func.o total.c | $(BIN_DIR)
	$(CC) -o $@ total.c asm_func.o

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


