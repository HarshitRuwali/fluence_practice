rm -f artifacts/* || true
mkdir -p artifacts

(
  cd local_storage || exit;
  marine build --release;
  cp target/wasm32-wasi/release/local_storage.wasm ../artifacts/

)

(
  cd curl_adapter || exit;
  marine build --release;
  cp target/wasm32-wasi/release/curl_adapter.wasm ../artifacts/
)

(
  cd facade || exit;
  marine build --release;
  cp target/wasm32-wasi/release/facade.wasm ../artifacts/
)



