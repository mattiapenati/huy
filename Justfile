_default:
  @just -l

# Check for errors
check:
  cargo check --tests --benches

# Run package unit test
test:
  cargo nextest run --no-fail-fast

# Run package benchmark
bench:
  cargo bench

# Generate the package documentation
doc:
  cargo doc --no-deps

# Serve the package documentation
serve-doc: doc
  uv run python -m http.server --directory target/doc
