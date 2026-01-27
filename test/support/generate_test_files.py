#!/usr/bin/env python3
"""
Generate test Excel files for Xler test suite.

Requirements: pip install openpyxl

Run: python test/support/generate_test_files.py
"""

import os
from pathlib import Path
from openpyxl import Workbook

# Get the project root directory
script_dir = Path(__file__).parent
test_data_dir = script_dir.parent / "data"

# Create test/data directory if it doesn't exist
test_data_dir.mkdir(parents=True, exist_ok=True)

def create_empty_sheet():
    """Create an Excel file with an empty worksheet."""
    wb = Workbook()
    ws = wb.active
    ws.title = "Sheet1"
    # Leave it empty - no data added
    filepath = test_data_dir / "empty_sheet.xlsx"
    wb.save(filepath)
    print(f"✓ Created {filepath}")

def create_single_cell():
    """Create an Excel file with a single cell containing 'test'."""
    wb = Workbook()
    ws = wb.active
    ws.title = "Sheet1"
    ws['A1'] = "test"
    filepath = test_data_dir / "single_cell.xlsx"
    wb.save(filepath)
    print(f"✓ Created {filepath}")

def create_multiple_sheets():
    """Create an Excel file with 3 different sheets."""
    wb = Workbook()

    # First sheet
    ws1 = wb.active
    ws1.title = "Sheet1"
    ws1.append(["ID", "Name", "Value"])
    ws1.append([1, "Alice", 100])
    ws1.append([2, "Bob", 200])

    # Second sheet
    ws2 = wb.create_sheet("Data")
    ws2.append(["Product", "Price", "Quantity"])
    ws2.append(["Widget", 10.50, 5])
    ws2.append(["Gadget", 25.00, 3])

    # Third sheet
    ws3 = wb.create_sheet("Summary")
    ws3.append(["Total Items", "Total Value"])
    ws3.append([8, 356.50])

    filepath = test_data_dir / "multiple_sheets.xlsx"
    wb.save(filepath)
    print(f"✓ Created {filepath}")

def create_unicode_data():
    """Create an Excel file with Unicode content including Japanese and emoji."""
    wb = Workbook()
    ws = wb.active
    ws.title = "Sheet1"
    ws.append(["Language", "Text"])
    ws.append(["English", "Hello World"])
    ws.append(["Japanese", "日本語"])
    ws.append(["French", "Bonjour"])
    ws.append(["Emoji", "🎉 émoji 🚀"])
    ws.append(["Chinese", "你好"])
    ws.append(["Arabic", "مرحبا"])

    filepath = test_data_dir / "unicode_data.xlsx"
    wb.save(filepath)
    print(f"✓ Created {filepath}")

def create_invalid_text_file():
    """Create a plain text file (not an Excel file) for testing error handling."""
    filepath = test_data_dir / "invalid.txt"
    with open(filepath, 'w') as f:
        f.write("This is not an Excel file.\n")
        f.write("It's just plain text.\n")
    print(f"✓ Created {filepath}")

def create_special_chars():
    """Create an Excel file with special characters in sheet names and data."""
    wb = Workbook()

    # Sheet with special characters (Excel has limitations on sheet names)
    ws = wb.active
    ws.title = "Data & Stats"
    ws.append(["Name", "Value", "Notes"])
    ws.append(["Item-1", "100", "Special: @#$%"])
    ws.append(["Item/2", "200", "Path: C:\\test\\file"])
    ws.append(["Item (3)", "300", "Brackets: [test]"])

    filepath = test_data_dir / "special_chars.xlsx"
    wb.save(filepath)
    print(f"✓ Created {filepath}")

def main():
    """Generate all test files."""
    print("Generating test Excel files...")
    print(f"Output directory: {test_data_dir.absolute()}\n")

    try:
        create_empty_sheet()
        create_single_cell()
        create_multiple_sheets()
        create_unicode_data()
        create_invalid_text_file()
        create_special_chars()

        print(f"\n✓ Successfully generated 6 test files in {test_data_dir}")
        print("\nFiles created:")
        print("  - empty_sheet.xlsx     (empty worksheet)")
        print("  - single_cell.xlsx     (single cell with 'test')")
        print("  - multiple_sheets.xlsx (3 sheets with sample data)")
        print("  - unicode_data.xlsx    (Unicode and emoji content)")
        print("  - invalid.txt          (plain text file)")
        print("  - special_chars.xlsx   (special characters in names)")

    except Exception as e:
        print(f"\n✗ Error generating test files: {e}")
        raise

if __name__ == "__main__":
    main()
