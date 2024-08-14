# Run rust examples
.PHONY: enum_marco enum_macro_darling
 
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

enum_macro:
	@RUST_LOG=info cargo run --example enum_macro

enum_macro_darling:
	@RUST_LOG=info cargo run --example enum_macro_darling