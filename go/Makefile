BIN := $(GOPATH)/bin/aoc
GOFILES := $(shell find . -type f -name '*.go')

$(BIN): $(GOFILES) ## Build the binary
	go1.18beta1 build -o $(BIN) .

clean: ## Remove build artifacts
	rm $(BIN)

tidy: ## Tidy the module
	go1.18beta1 mod tidy

.PHONY: build clean tidy
