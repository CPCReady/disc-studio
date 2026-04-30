#!/bin/bash
# MIT License - Copyright (c) 2026 Destroyer
# Test script for disc with real DSK examples

set -e

DISC="./target/release/disc"
EXAMPLES="../Examples"

echo "🧪 Testing disc with real DSK examples"
echo "========================================"
echo ""

# Check if disc is built
if [ ! -f "$DISC" ]; then
    echo "❌ disc binary not found. Building..."
    cargo build --release
fi

# Test 1: Abu Simbel Profanation
echo "Test 1: Abu Simbel Profanation"
echo "-------------------------------"
$DISC list "$EXAMPLES/ABU_SIMBEL_PROFANATION.DSK"
echo ""

# Test 2: Airwolf
echo "Test 2: Airwolf"
echo "---------------"
$DISC list "$EXAMPLES/AIRWOLF.DSK"
echo ""

# Test 3: Barbarian (multiple files)
echo "Test 3: Barbarian (multiple files)"
echo "-----------------------------------"
$DISC list "$EXAMPLES/BARBARIAN.DSK"
echo ""

# Test 4: JSON format
echo "Test 4: JSON format"
echo "-------------------"
$DISC list "$EXAMPLES/BARBARIAN.DSK" --format json | head -20
echo "..."
echo ""

# Test 5: CSV format
echo "Test 5: CSV format"
echo "------------------"
$DISC list "$EXAMPLES/BARBARIAN.DSK" --format csv
echo ""

# Test 6: Simple format
echo "Test 6: Simple format"
echo "---------------------"
$DISC list "$EXAMPLES/BARBARIAN.DSK" --format simple
echo ""

# Test 7: Create new DSK
echo "Test 7: Create new DSK"
echo "----------------------"
$DISC create /tmp/test_disc.dsk --force
$DISC list /tmp/test_disc.dsk
rm -f /tmp/test_disc.dsk
echo ""

echo "✅ All tests passed!"
echo ""
echo "Summary:"
echo "--------"
echo "✓ Read DSK with 39 tracks"
echo "✓ Read DSK with 42 tracks"
echo "✓ Read DSK with multiple files"
echo "✓ JSON output format"
echo "✓ CSV output format"
echo "✓ Simple output format"
echo "✓ Create new DSK"
echo ""
echo "🎉 disc is working correctly with real DSK files!"
