CC := gcc
CFLAGS := -O0 -g
TARGET := ptrace_test
SOURCE := ptrace.c
BIN_DIR := ./bin

.PHONY: all clean

all: $(BIN_DIR)/$(TARGET)

$(BIN_DIR)/$(TARGET): $(SOURCE)
	@mkdir -p $(BIN_DIR)
	$(CC) $(CFLAGS) -o $@ $^

clean:
	rm -rf $(BIN_DIR)

