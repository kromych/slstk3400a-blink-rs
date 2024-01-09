#/bin/sh

set -e

# apply the patch with device info:
svdtools patch ./EFM32HG322F64.yaml

# produce the Rust descriptions:
svd2rust -i EFM32HG322F64.svd.patched --target cortex-m --atomics --atomics-feature "portable-atomics" --reexport-core-peripherals --reexport-interrupt

# split the large `lib.rs` file into modules:
form -i ./lib.rs -o ./src

# Clean up and situate the files:
mv ./build.rs ../
mv ./device.x ../
rm -rf ../src
mkdir ../src
mv ./src ../
