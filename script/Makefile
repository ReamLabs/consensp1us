TARGET = mainnet.tar.gz
EXTRACT_DIR = mainnet
LOGS_DIR = logs

DOWNLOAD_SCRIPT = ./subscripts/download_ef_data.sh
PARSE_SCRIPT = ./subscripts/parse_log_to_table.sh

.PHONY: all download run clean

all: download run

download:
	@echo "Running download script..."
	@chmod +x $(DOWNLOAD_SCRIPT)
	@$(DOWNLOAD_SCRIPT)

run: $(EXTRACT_DIR)
	@mkdir -p $(LOGS_DIR)
	@echo "Running benchmarks..."
	@NO_COLOR=1 cargo run --release -- --execute -o attestation &> logs/execution_attestation.log
	@echo "Execution complete."
	@$(PARSE_SCRIPT) attestation

clean:
	@echo "Cleaning up downloaded/execution files..."
	@rm -f $(TARGET)
	@rm -rf $(EXTRACT_DIR)
	@rm -rf $(LOGS_DIR)
	@echo "Clean up complete."