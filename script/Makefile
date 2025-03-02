TARGET = mainnet.tar.gz
EXTRACT_DIR = mainnet
SCRIPT = ./subscripts/download_ef_data.sh

.PHONY: all download test clean

all: download test

download:
	@echo "Running download script..."
	@chmod +x $(SCRIPT)
	@$(SCRIPT)

test: $(EXTRACT_DIR)
	@echo "Running tests..."
	@cargo test --release --features ef-tests
	@echo "Tests complete."

clean:
	@echo "Cleaning up downloaded and extracted files..."
	@rm -f $(TARGET)
	@rm -rf $(EXTRACT_DIR)
	@echo "Clean up complete."