#!/bin/bash

# Create a realistic boot animation using terminal recording simulation
# This creates an animated GIF showing the boot sequence

echo "Creating boot-demo.gif..."

# Create frames directory
mkdir -p assets/frames

# Generate 30 frames of boot sequence
for i in {1..30}; do
    # Calculate progress
    progress=$((i * 100 / 30))
    
    # Create frame content
    cat > assets/frames/frame_$(printf "%03d" $i).txt << EOF
[0;31m[1mVANTIS OS v5.0 - Secure Boot Sequence[0m

[BOOT]  Initializing kernel...${progress}%
[SEC]   Formal verification: PASS
[SEC]   SLSA Level 4: Verified
[BOOT]  Vantis Vault: Initialized
[BOOT]  Neural Scheduler: Active
[BOOT]  Sentinel Drivers: Loaded
[IPC]   Zero-Copy IPC: Online (<1μs latency)
[BOOT]  Flux Engine: Ready
[BOOT]  Horizon UI: Starting...

${progress}% complete - Loading verified modules...
EOF
    
    # Add frame number for debugging
    echo "Created frame $i/30"
done

echo "Frames created. Converting to GIF..."
echo "To convert frames to GIF, you need ImageMagick:"
echo "  convert -delay 20 assets/frames/frame_*.txt assets/boot-demo.gif"
echo ""
echo "For now, use assets/boot-demo.cast for ASCIINEMA player"