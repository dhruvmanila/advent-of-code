BIN := $(GOPATH)/bin/aoc

build: ## Build the binary
	go build -o $(BIN) .

clean: ## Remove build artifacts
	rm $(BIN)

tidy: ## Tidy the module
	go mod tidy

.PHONY: build clean tidy help
