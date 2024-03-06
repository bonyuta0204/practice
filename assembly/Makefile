NASM := nasm

TARGET := hello

BIN_DIR := ./bin


%.o: %.s
	$(NASM) -f elf64 -o $@ $<

$(BIN_DIR)/$(TARGET): $(TARGET).o
	@mkdir -p $(BIN_DIR)
	$(LD) -o $@ $^

clean:
	rm -rf $(BIN_DIR) *.o

