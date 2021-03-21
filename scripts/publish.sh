#! /bin/bash
set -euo pipefail

echo Publishing credent_model &&
(cd crate/credent_model     && cargo publish && sleep 25) &&
\
echo Publishing credent_cli_model &&
(cd crate/credent_cli_model && cargo publish --features "backend-smol" && sleep 25) &&
\
echo Publishing credent_cli &&
(cd crate/credent_cli       && cargo publish --features "backend-smol" && sleep 25) &&
\
echo Publishing credent_fs_model &&
(cd crate/credent_fs_model  && cargo publish && sleep 25) &&
\
echo Publishing credent_fs &&
(cd crate/credent_fs        && cargo publish && sleep 25) &&
\
echo Publishing credent &&
cargo publish --features "backend-smol"
