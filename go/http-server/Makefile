.PHONY: build test lint run clean

# Build the application
build: build-server build-client

build-server:
	mkdir -p bin
	go build -o bin/server ./cmd/server

build-client:
	mkdir -p bin
	go build -o bin/client ./cmd/client

# Run all tests
test:
	go test ./... -v

# Run linter
lint:
	go vet ./...
	@if command -v golangci-lint >/dev/null 2>&1; then \
		golangci-lint run; \
	else \
		echo "golangci-lint is not installed. Run: go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest"; \
	fi

# Run the server
run: build-server
	./bin/server

# Clean build artifacts
clean:
	rm -rf bin/*
