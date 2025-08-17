set -euo pipefail

OUT="support-bundle.txt"
echo "# === Amazing Korean Support Bundle ===" > "$OUT"

echo -e "\n## rust/cargo version" >> "$OUT"
rustc -V >> "$OUT" 2>&1 || true
cargo -V >> "$OUT" 2>&1 || true

echo -e "\n## cargo tree (features)" >> "$OUT"
cargo tree -e features >> "$OUT" 2>&1 || true

echo -e "\n## project tree (L3)" >> "$OUT"
command -v tree >/dev/null && tree -I 'target|.git|node_modules' -L 3 >> "$OUT" \
  || find . -maxdepth 3 -type f | sed 's#^\./##' >> "$OUT"

echo -e "\n## Cargo.toml" >> "$OUT"
cat Cargo.toml >> "$OUT" 2>&1 || true

echo -e "\n## rust-toolchain.toml" >> "$OUT"
[ -f rust-toolchain.toml ] && cat rust-toolchain.toml >> "$OUT" || echo "(none)" >> "$OUT"

echo -e "\n## .env (masked)" >> "$OUT"
if [ -f .env ]; then
  sed -E 's/(JWT_SECRET=).*/\1<REDACTED>/; s#(DATABASE_URL=).*#\1<REDACTED>#' .env >> "$OUT"
else
  echo "(no .env; create .env as shown in the guide)" >> "$OUT"
fi

echo -e "\n## Key source files" >> "$OUT"
for f in \
  src/main.rs \
  src/docs.rs \
  src/error.rs \
  src/state.rs \
  src/api/mod.rs \
  src/api/app_router.rs \
  src/api/auth/dto.rs \
  src/api/auth/handler.rs \
  src/api/auth/service.rs \
  src/api/course/dto.rs \
  src/api/course/handler.rs \
  src/api/course/service.rs \
  ; do
  if [ -f "$f" ]; then
    echo -e "\n### FILE: $f\n" >> "$OUT"
    sed -n '1,300p' "$f" >> "$OUT"
  fi
done

echo -e "\n## last cargo check output" >> "$OUT"
cargo check >> "$OUT" 2>&1 || true

echo "Done. -> $OUT"
