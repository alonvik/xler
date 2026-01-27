defmodule XlerTest do
  use ExUnit.Case
  # doctest Xler

  def file(), do: File.cwd!() <> "/test/data/sample.xlsx"

  # Helper function for test data files
  defp test_file(name), do: Path.join([File.cwd!(), "test", "data", name])

  describe "happy path" do
    test "can read worksheets from xlsx file" do
      assert Xler.worksheets(file()) == {:ok, ["Sheet1"]}
    end

    test "gets a list of data returned via the worksheet name" do
      {:ok, data} = Xler.parse(file(), "Sheet1")

      assert length(data) == 701
    end

    test "returns the correct first row" do
      {:ok, data} = Xler.parse(file(), "Sheet1")

      assert data |> List.first() == [
               "Segment",
               "Country",
               "Product",
               "Discount Band",
               "Units Sold",
               "Manufacturing Price",
               "Sale Price",
               "Gross Sales",
               "Discounts",
               " Sales",
               "COGS",
               "Profit",
               "Date",
               "Month Number",
               "Month Name",
               "Year"
             ]
    end

    test "can read multiple sheets" do
      assert {:ok, sheets} = Xler.worksheets(test_file("multiple_sheets.xlsx"))
      assert length(sheets) == 3
      assert "Sheet1" in sheets
      assert "Data" in sheets
      assert "Summary" in sheets
    end

    test "can parse empty worksheet" do
      assert {:ok, data} = Xler.worksheets(test_file("empty_sheet.xlsx"))
      assert data == ["Sheet1"]
    end

    test "can parse single cell" do
      assert {:ok, data} = Xler.parse(test_file("single_cell.xlsx"), "Sheet1")
      assert data == [["test"]]
    end

    test "can parse unicode content" do
      assert {:ok, data} = Xler.parse(test_file("unicode_data.xlsx"), "Sheet1")
      assert length(data) > 0
      # Verify Unicode is preserved in the data
      assert Enum.any?(data, fn row -> Enum.any?(row, &String.contains?(&1, "日本語")) end)
    end
  end

  describe "worksheets/1 error handling" do
    test "returns error for nil input" do
      assert {:error, _} = Xler.worksheets(nil)
    end

    test "returns error for integer input" do
      assert {:error, _} = Xler.worksheets(12345)
    end

    test "returns error for atom input" do
      assert {:error, _} = Xler.worksheets(:some_atom)
    end

    test "returns error for list input" do
      assert {:error, _} = Xler.worksheets([1, 2, 3])
    end

    test "returns error for map input" do
      assert {:error, _} = Xler.worksheets(%{key: "value"})
    end

    test "returns error for non-existent file" do
      assert {:error, msg} = Xler.worksheets(test_file("nonexistent.xlsx"))
      assert is_binary(msg)
    end

    test "returns error for empty string filename" do
      assert {:error, msg} = Xler.worksheets("")
      assert is_binary(msg)
    end

    test "returns error for non-Excel file (text file)" do
      assert {:error, msg} = Xler.worksheets(test_file("invalid.txt"))
      assert is_binary(msg)
    end
  end

  describe "parse/2 error handling" do
    test "returns error for nil filename" do
      assert {:error, _} = Xler.parse(nil, "Sheet1")
    end

    test "returns error for integer filename" do
      assert {:error, _} = Xler.parse(12345, "Sheet1")
    end

    test "returns error for non-string filename" do
      assert {:error, _} = Xler.parse(:atom, "Sheet1")
    end

    test "returns error for non-existent file" do
      assert {:error, msg} = Xler.parse(test_file("nonexistent.xlsx"), "Sheet1")
      assert is_binary(msg)
    end

    test "returns error for nil worksheet name" do
      assert {:error, _} = Xler.parse(file(), nil)
    end

    test "returns error for integer worksheet name" do
      assert {:error, _} = Xler.parse(file(), 123)
    end

    test "returns error for non-string worksheet name" do
      assert {:error, _} = Xler.parse(file(), :atom)
    end

    test "returns error for empty worksheet name" do
      assert {:error, msg} = Xler.parse(file(), "")
      assert is_binary(msg)
    end

    test "returns error for worksheet that doesn't exist in file" do
      assert {:error, msg} = Xler.parse(file(), "NonExistentSheet")
      assert is_binary(msg)
    end

    test "returns error for non-Excel file" do
      assert {:error, msg} = Xler.parse(test_file("invalid.txt"), "Sheet1")
      assert is_binary(msg)
    end
  end

  describe "multiple worksheets" do
    test "can parse each worksheet from multi-sheet file individually" do
      {:ok, data1} = Xler.parse(test_file("multiple_sheets.xlsx"), "Sheet1")
      {:ok, data2} = Xler.parse(test_file("multiple_sheets.xlsx"), "Data")
      {:ok, data3} = Xler.parse(test_file("multiple_sheets.xlsx"), "Summary")

      assert length(data1) > 0
      assert length(data2) > 0
      assert length(data3) > 0
    end
  end

  describe "special characters" do
    test "can read worksheet with special characters in sheet name" do
      assert {:ok, sheets} = Xler.worksheets(test_file("special_chars.xlsx"))
      assert "Data & Stats" in sheets
    end

    test "can parse worksheet with special characters in sheet name" do
      assert {:ok, data} = Xler.parse(test_file("special_chars.xlsx"), "Data & Stats")
      assert length(data) > 0
    end
  end
end
