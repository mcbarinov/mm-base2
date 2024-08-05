clean:
  rm -rf target/


lint:
  cargo fmt && cargo clippy
