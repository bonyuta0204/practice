CC := gcc
CFLAGS := -O0 -g
TARGET := mtrace
SOURCE := mtrace.c
BIN_DIR := ./bin

.PHONY: all clean

all: $(BIN_DIR)/$(TARGET)

$(BIN_DIR)/$(TARGET): $(SOURCE)
	@mkdir -p $(BIN_DIR)
	$(CC) $(CFLAGS) -o $@ $^

%.s: %.c
	$(CC) -masm=intel -S -o $@ $^

clean:
	rm -rf $(BIN_DIR) *.s

