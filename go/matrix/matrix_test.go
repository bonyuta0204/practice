package main

import (
	"fmt"
	"reflect"
	"testing"
)

func TestMatrixDataBuilder_Build(t *testing.T) {
	tests := []struct {
		name         string
		rawData      [][]string
		headerParts  []string
		expected     *MatrixData
		expectNil    bool
	}{
		{
			name: "valid data with 2 header parts",
			rawData: [][]string{
				{"", "Header1", "", "Header2", ""},
				{"", "Part1", "Part2", "Part1", "Part2"},
				{"Row1", "1", "2", "3", "4"},
				{"Row2", "5", "6", "7", "8"},
			},
			headerParts: []string{"Part1", "Part2"},
			expected: &MatrixData{
				columnHeaders:  []string{"Header1", "Header2"},
				columnSubHeaders: []string{"Part1", "Part2"},
				rowHeaders:  []string{"Row1", "Row2"},
				values: [][][]string{
					{{"1", "2"}, {"3", "4"}},
					{{"5", "6"}, {"7", "8"}},
				},
			},
		},
		{
			name: "valid data with 3 header parts",
			rawData: [][]string{
				{"", "H1", "", "", "H2", "", ""},
				{"", "P1", "P2", "P3", "P1", "P2", "P3"},
				{"R1", "a", "b", "c", "d", "e", "f"},
			},
			headerParts: []string{"P1", "P2", "P3"},
			expected: &MatrixData{
				columnHeaders:  []string{"H1", "H2"},
				columnSubHeaders: []string{"P1", "P2", "P3"},
				rowHeaders:  []string{"R1"},
				values: [][][]string{
					{{"a", "b", "c"}, {"d", "e", "f"}},
				},
			},
		},
		{
			name: "empty header in first row",
			rawData: [][]string{
				{"", "", "", "Header2", ""},
				{"", "Part1", "Part2", "Part1", "Part2"},
				{"Row1", "1", "2", "3", "4"},
			},
			headerParts: []string{"Part1", "Part2"},
			expectNil:   true,
		},
		{
			name: "invalid second row",
			rawData: [][]string{
				{"", "Header1", "", "Header2", ""},
				{"", "Part1", "Wrong", "Part1", "Part2"},
				{"Row1", "1", "2", "3", "4"},
			},
			headerParts: []string{"Part1", "Part2"},
			expectNil:   true,
		},
		{
			name: "row with insufficient data",
			rawData: [][]string{
				{"", "Header1", ""},
				{"", "Part1"},
				{"Row1"},
			},
			headerParts: []string{"Part1"},
			expectNil:   true,
		},
	}

	builder := &MatrixDataBuilder{}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := builder.Build(tt.rawData, tt.headerParts)
			if tt.expectNil {
				if result != nil {
					t.Errorf("expected nil result, got %v", result)
				}
			} else {
				if result == nil {
					t.Fatal("expected non-nil result, got nil")
				}
				if !reflect.DeepEqual(result, tt.expected) {
					t.Errorf("result mismatch\ngot:  %+v\nwant: %+v", result, tt.expected)
				}
			}
		})
	}
}

func TestMatrixDataBuilder_parseFirstRow(t *testing.T) {
	tests := []struct {
		name        string
		row         []string
		partCount   int
		expected    []string
		expectError bool
	}{
		{
			name:      "valid row with 2 parts",
			row:       []string{"", "H1", "", "H2", ""},
			partCount: 2,
			expected:  []string{"H1", "H2"},
		},
		{
			name:      "valid row with 3 parts",
			row:       []string{"", "Header1", "", "", "Header2", "", ""},
			partCount: 3,
			expected:  []string{"Header1", "Header2"},
		},
		{
			name:        "empty header",
			row:         []string{"", "", "", "H2", ""},
			partCount:   2,
			expectError: true,
		},
		{
			name:      "single header",
			row:       []string{"", "OnlyOne"},
			partCount: 1,
			expected:  []string{"OnlyOne"},
		},
		{
			name:      "empty row",
			row:       []string{""},
			partCount: 1,
			expected:  []string{},
		},
	}

	builder := &MatrixDataBuilder{}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result, err := builder.parseFirstRow(tt.row, tt.partCount)
			if tt.expectError {
				if err == nil {
					t.Error("expected error, got nil")
				}
			} else {
				if err != nil {
					t.Errorf("unexpected error: %v", err)
				}
				if !reflect.DeepEqual(result, tt.expected) {
					t.Errorf("result mismatch\ngot:  %v\nwant: %v", result, tt.expected)
				}
			}
		})
	}
}

func TestMatrixDataBuilder_validateSecondRow(t *testing.T) {
	tests := []struct {
		name        string
		row         []string
		headerParts []string
		expectError bool
	}{
		{
			name:        "valid row",
			row:         []string{"", "Part1", "Part2", "Part1", "Part2"},
			headerParts: []string{"Part1", "Part2"},
		},
		{
			name:        "valid row with 3 parts",
			row:         []string{"", "A", "B", "C", "A", "B", "C"},
			headerParts: []string{"A", "B", "C"},
		},
		{
			name:        "invalid part",
			row:         []string{"", "Part1", "Wrong", "Part1", "Part2"},
			headerParts: []string{"Part1", "Part2"},
			expectError: true,
		},
		{
			name:        "single part",
			row:         []string{"", "Single", "Single", "Single"},
			headerParts: []string{"Single"},
		},
		{
			name:        "out of order parts",
			row:         []string{"", "Part2", "Part1", "Part1", "Part2"},
			headerParts: []string{"Part1", "Part2"},
			expectError: true,
		},
	}

	builder := &MatrixDataBuilder{}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := builder.validateSecondRow(tt.row, tt.headerParts)
			if tt.expectError {
				if err == nil {
					t.Error("expected error, got nil")
				}
			} else {
				if err != nil {
					t.Errorf("unexpected error: %v", err)
				}
			}
		})
	}
}

func TestMatrixDataBuilder_BuildParallel(t *testing.T) {
	tests := []struct {
		name         string
		rawData      [][]string
		headerParts  []string
		expected     *MatrixData
		expectNil    bool
	}{
		{
			name: "valid data with 2 header parts",
			rawData: [][]string{
				{"", "Header1", "", "Header2", ""},
				{"", "Part1", "Part2", "Part1", "Part2"},
				{"Row1", "1", "2", "3", "4"},
				{"Row2", "5", "6", "7", "8"},
			},
			headerParts: []string{"Part1", "Part2"},
			expected: &MatrixData{
				columnHeaders:    []string{"Header1", "Header2"},
				columnSubHeaders: []string{"Part1", "Part2"},
				rowHeaders:       []string{"Row1", "Row2"},
				values: [][][]string{
					{{"1", "2"}, {"3", "4"}},
					{{"5", "6"}, {"7", "8"}},
				},
			},
		},
		{
			name: "large data set",
			rawData: func() [][]string {
				data := [][]string{
					{"", "H1", "", "H2", "", "H3", ""},
					{"", "P1", "P2", "P1", "P2", "P1", "P2"},
				}
				for i := 0; i < 100; i++ {
					row := []string{fmt.Sprintf("Row%d", i)}
					for j := 0; j < 6; j++ {
						row = append(row, fmt.Sprintf("%d-%d", i, j))
					}
					data = append(data, row)
				}
				return data
			}(),
			headerParts: []string{"P1", "P2"},
			expectNil:   false,
		},
	}

	builder := &MatrixDataBuilder{}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := builder.BuildParallel(tt.rawData, tt.headerParts)
			if tt.expectNil {
				if result != nil {
					t.Errorf("expected nil result, got %v", result)
				}
			} else {
				if result == nil {
					t.Fatal("expected non-nil result, got nil")
				}
				// For large data set, just check basic properties
				if tt.name == "large data set" {
					if len(result.rowHeaders) != 100 {
						t.Errorf("expected 100 row headers, got %d", len(result.rowHeaders))
					}
					if len(result.values) != 100 {
						t.Errorf("expected 100 value rows, got %d", len(result.values))
					}
				} else if tt.expected != nil {
					if !reflect.DeepEqual(result, tt.expected) {
						t.Errorf("result mismatch\ngot:  %+v\nwant: %+v", result, tt.expected)
					}
				}
			}
		})
	}
}

func TestMatrixDataBuilder_parseRow(t *testing.T) {
	tests := []struct {
		name           string
		row            []string
		partSize       int
		expectedHeader string
		expectedData   [][]string
		expectError    bool
	}{
		{
			name:           "valid row with partSize 2",
			row:            []string{"Header", "1", "2", "3", "4"},
			partSize:       2,
			expectedHeader: "Header",
			expectedData:   [][]string{{"1", "2"}, {"3", "4"}},
		},
		{
			name:           "valid row with partSize 3",
			row:            []string{"H", "a", "b", "c", "d", "e", "f"},
			partSize:       3,
			expectedHeader: "H",
			expectedData:   [][]string{{"a", "b", "c"}, {"d", "e", "f"}},
		},
		{
			name:           "row with uneven parts",
			row:            []string{"Header", "1", "2", "3"},
			partSize:       2,
			expectedHeader: "Header",
			expectedData:   [][]string{{"1", "2"}, {"3"}},
		},
		{
			name:           "row with single data element",
			row:            []string{"H", "data"},
			partSize:       1,
			expectedHeader: "H",
			expectedData:   [][]string{{"data"}},
		},
		{
			name:        "empty row",
			row:         []string{},
			partSize:    2,
			expectError: true,
		},
		{
			name:        "single element row",
			row:         []string{"OnlyHeader"},
			partSize:    2,
			expectError: true,
		},
		{
			name:           "row with empty data",
			row:            []string{"Header", "", "", ""},
			partSize:       2,
			expectedHeader: "Header",
			expectedData:   [][]string{{"", ""}, {""}},
		},
	}

	builder := &MatrixDataBuilder{}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			header, data, err := builder.parseRow(tt.row, tt.partSize)
			if tt.expectError {
				if err == nil {
					t.Error("expected error, got nil")
				}
			} else {
				if err != nil {
					t.Errorf("unexpected error: %v", err)
				}
				if header != tt.expectedHeader {
					t.Errorf("header mismatch\ngot:  %v\nwant: %v", header, tt.expectedHeader)
				}
				if !reflect.DeepEqual(data, tt.expectedData) {
					t.Errorf("data mismatch\ngot:  %v\nwant: %v", data, tt.expectedData)
				}
			}
		})
	}
}