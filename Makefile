# Run rust examples
.PHONY: tracing
 
# clap:
# 	cargo run --example clap -- $(ARGS) # USAGE: make clap ARGS="--help"

# subcommand:
# 	cargo run --example subcommand -- $(ARGS)

# mode:
# 	cargo run --example mode -- $(ARGS)

# validated_values:
# 	cargo run --example validated_values -- $(ARGS)

# text-cli:
# 	cargo run --example text_cli -- $(ARGS)
# async:
# 	cargo run --example async
# async2:
# 	cargo run --example async2
# axum:
# 	cargo run --example axum
# thread: 
# 	cargo run --example thread

# run:
# 	@cargo run -- $(ARGS)

# run_with_log:
# 	@RUST_LOG=info cargo run -- $(ARGS)

tracing_info:
	@RUST_LOG=info cargo run --example tracing

tracing_trace:
	@RUST_LOG=trace cargo run --example tracing

axum_tracing:
	@RUST_LOG=info cargo run --example axum_tracing