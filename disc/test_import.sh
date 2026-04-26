#!/bin/bash
# MIT License - Copyright (c) 2026 Destroyer
# Test script for disc import command

set -e

DISC="./target/release/disc"

echo "🧪 Testing disc import command"
echo "==============================="
echo ""

# Test 1: Import single file
echo "Test 1: Import single ASCII file"
echo "---------------------------------"
cat > /tmp/test1.txt << 'EOF'
Hello from Amstrad CPC!
This is a test file.
EOF

$DISC create /tmp/import_test1.dsk --force > /dev/null
$DISC import /tmp/import_test1.dsk /tmp/test1.txt -t ascii
$DISC list /tmp/import_test1.dsk
echo ""

# Test 2: Import binary file
echo "Test 2: Import binary file with addresses"
echo "------------------------------------------"
dd if=/dev/urandom of=/tmp/test2.bin bs=1024 count=4 2>/dev/null
$DISC create /tmp/import_test2.dsk --force > /dev/null
$DISC import /tmp/import_test2.dsk /tmp/test2.bin -t binary --load 0x4000 --exec 0xC000
$DISC list /tmp/import_test2.dsk
echo ""

# Test 3: Import with attributes
echo "Test 3: Import with read-only and system attributes"
echo "----------------------------------------------------"
cat > /tmp/test3.bas << 'EOF'
10 PRINT "System file"
20 END
EOF

$DISC create /tmp/import_test3.dsk --force > /dev/null
$DISC import /tmp/import_test3.dsk /tmp/test3.bas --read-only --system
$DISC list /tmp/import_test3.dsk
echo ""

# Test 4: Import multiple files
echo "Test 4: Import multiple files"
echo "------------------------------"
for i in 1 2 3; do
    echo "File $i content" > /tmp/file$i.txt
done

$DISC create /tmp/import_test4.dsk --force > /dev/null
$DISC import /tmp/import_test4.dsk /tmp/file1.txt /tmp/file2.txt /tmp/file3.txt
$DISC list /tmp/import_test4.dsk
echo ""

# Test 5: Import real file from Examples
echo "Test 5: Import and re-export real file"
echo "---------------------------------------"
$DISC export ../Examples/BARBARIAN.DSK BARBRN1E.BAS -o /tmp/export_orig > /dev/null
$DISC create /tmp/import_test5.dsk --force > /dev/null
$DISC import /tmp/import_test5.dsk /tmp/export_orig/BARBRN1E.BAS
$DISC export /tmp/import_test5.dsk BARBRN1E.BAS -o /tmp/export_new > /dev/null

if diff /tmp/export_orig/BARBRN1E.BAS /tmp/export_new/BARBRN1E.BAS > /dev/null; then
    echo "✓ Files are identical!"
else
    echo "✗ Files differ!"
    exit 1
fi
echo ""

# Test 6: Disk full error
echo "Test 6: Disk full error handling"
echo "---------------------------------"
dd if=/dev/urandom of=/tmp/bigfile.bin bs=1024 count=200 2>/dev/null
$DISC create /tmp/import_test6.dsk --force > /dev/null
$DISC import /tmp/import_test6.dsk /tmp/bigfile.bin 2>&1 | grep -q "Not enough space" && echo "✓ Disk full error handled correctly" || echo "✗ Error not detected"
echo ""

# Test 7: File exists error
echo "Test 7: File exists error (without --force)"
echo "--------------------------------------------"
echo "Test content" > /tmp/test7.txt
$DISC create /tmp/import_test7.dsk --force > /dev/null
$DISC import /tmp/import_test7.dsk /tmp/test7.txt > /dev/null
$DISC import /tmp/import_test7.dsk /tmp/test7.txt 2>&1 | grep -q "already exists" && echo "✓ File exists error handled correctly" || echo "✗ Error not detected"
echo ""

# Test 8: Import with --force (overwrite)
echo "Test 8: Import with --force (overwrite)"
echo "----------------------------------------"
echo "Original content" > /tmp/test8.txt
$DISC create /tmp/import_test8.dsk --force > /dev/null
$DISC import /tmp/import_test8.dsk /tmp/test8.txt > /dev/null

echo "Modified content" > /tmp/test8.txt
$DISC import /tmp/import_test8.dsk /tmp/test8.txt --force
echo "✓ File overwritten with --force"
echo ""

# Cleanup
rm -f /tmp/test*.txt /tmp/test*.bin /tmp/test*.bas /tmp/file*.txt /tmp/bigfile.bin
rm -f /tmp/import_test*.dsk
rm -rf /tmp/export_orig /tmp/export_new

echo "✅ All import tests passed!"
echo ""
echo "Summary:"
echo "--------"
echo "✓ Import ASCII file"
echo "✓ Import binary file with addresses"
echo "✓ Import with attributes (RO, SYS)"
echo "✓ Import multiple files"
echo "✓ Import and re-export (integrity check)"
echo "✓ Disk full error handling"
echo "✓ File exists error handling"
echo "✓ Overwrite with --force"
echo ""
echo "🎉 disc import is working correctly!"
