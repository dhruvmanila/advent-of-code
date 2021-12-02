BIN := aoc

build: ## Build the binary
	go build -o $(BIN) .

clean: ## Remove build artifacts
	rm $(BIN)

ignore: ## Add the executable file to .gitignore
	echo $(BIN) > .gitignore

.PHONY: build clean ignore
