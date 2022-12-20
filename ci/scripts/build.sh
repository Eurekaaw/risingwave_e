#!/bin/bash

# Exits as soon as any line fails.
set -euo pipefail

source ci/scripts/common.env.sh

while getopts 't:p:' opt; do
    case ${opt} in
        t )
            target=$OPTARG
            ;;
        p )
            profile=$OPTARG
            ;;
        \? )
            echo "Invalid Option: -$OPTARG" 1>&2
            exit 1
            ;;
        : )
            echo "Invalid option: $OPTARG requires an argument" 1>&2
            ;;
    esac
done
shift $((OPTIND -1))

echo "--- Rust cargo-sort check"
cargo sort --check --workspace

echo "--- Rust cargo-hakari check"
cargo hakari generate --diff

echo "--- Rust format check"
cargo fmt --all -- --check

echo "--- Build Rust components"
cargo build \
    -p risingwave_cmd_all \
    -p risedev \
    -p risingwave_regress_test \
    -p risingwave_sqlsmith \
    -p risingwave_compaction_test \
    -p risingwave_backup_cmd \
    --features "static-link static-log-level" --profile "$profile"

echo "--- Compress debug info for artifacts"
objcopy --compress-debug-sections=zlib-gnu target/"$target"/risingwave
objcopy --compress-debug-sections=zlib-gnu target/"$target"/sqlsmith
objcopy --compress-debug-sections=zlib-gnu target/"$target"/compaction-test
objcopy --compress-debug-sections=zlib-gnu target/"$target"/backup-restore
objcopy --compress-debug-sections=zlib-gnu target/"$target"/risingwave_regress_test
objcopy --compress-debug-sections=zlib-gnu target/"$target"/risedev-dev
objcopy --compress-debug-sections=zlib-gnu target/"$target"/delete-range-test

echo "--- Show link info"
ldd target/"$target"/risingwave

echo "--- Upload artifacts"
cp target/"$target"/compaction-test ./compaction-test-"$profile"
cp target/"$target"/backup-restore ./backup-restore-"$profile"
cp target/"$target"/risingwave ./risingwave-"$profile"
cp target/"$target"/risedev-dev ./risedev-dev-"$profile"
cp target/"$target"/risingwave_regress_test ./risingwave_regress_test-"$profile"
cp target/"$target"/sqlsmith ./sqlsmith-"$profile"
cp target/"$target"/delete-range-test ./delete-range-test-"$profile"
buildkite-agent artifact upload risingwave-"$profile"
buildkite-agent artifact upload risedev-dev-"$profile"
buildkite-agent artifact upload risingwave_regress_test-"$profile"
buildkite-agent artifact upload ./sqlsmith-"$profile"
buildkite-agent artifact upload ./compaction-test-"$profile"
buildkite-agent artifact upload ./backup-restore-"$profile"
buildkite-agent artifact upload ./delete-range-test-"$profile"
