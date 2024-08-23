ENDPOINT ?= mainnet.sol.streamingfast.io:443
DSN ?= clickhouse://default:@localhost:18123/default

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: stream
stream: build
	substreams run -e $(ENDPOINT) substreams.yaml map_trades -s 281251436 -t +1

.PHONY: stream_db_out
stream_db_out: build
	substreams run -e $(ENDPOINT) substreams.yaml db_out -t +10

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: stream_db
stream_db: build
	echo DSN=$(DNS)
	substreams-sink-sql setup "$(DSN)" --ignore-duplicate-table-errors ./substreams.yaml
	substreams-sink-sql run "$(DSN)" ./substreams.yaml

.PHONY: package
package:
	substreams pack ./substreams.yaml
