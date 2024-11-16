#!/bin/bash

rm -rf ./bin
# Define targets
targets=(
    "x86_64-unknown-linux-gnu"
    "x86_64-pc-windows-gnu"
)

# Define output directory for binaries
output_dir="bin"

# Create the output directory if it doesn't exist
mkdir -p "$output_dir"

# Iterate over targets and build
for target in "${targets[@]}"; do
    # Build the project for the target
    cargo build --target $target --release
    if [ $? -ne 0 ]; then
        echo "Build failed for target $target"
        exit 1
    fi

    # Extract the platform name (this will be used as part of the file name)
    platform_name=$(echo $target | sed 's/[^a-zA-Z0-9]/_/g')  # Replace non-alphanumeric characters with underscores

    # Define the output executable name based on the target
    executable_name="clua_$platform_name"

    # Move the built executable to the bin/ directory with the correct name
    if [ "$target" == "x86_64-unknown-linux-gnu" ]; then
        cp "target/$target/release/clua" "$output_dir/clua.out"
    elif [ "$target" == "x86_64-pc-windows-gnu" ]; then
        cp "target/$target/release/clua.exe" "$output_dir/clua.exe"
    fi

    # Check if the file was copied successfully
    if [ $? -ne 0 ]; then
        echo "Failed to copy executable for $target"
        exit 1
    fi
done

echo "Build completed successfully for all targets!"
