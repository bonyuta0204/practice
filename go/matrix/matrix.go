package main

import (
	"fmt"
)

type MatrixData struct {
	columnHeaders    []string
	columnSubHeaders []string
	rowHeaders       []string
	values           [][][]string
}

type MatrixDataBuilder struct {
}

func (b *MatrixDataBuilder) Build(rawData [][]string, headerParts []string) *MatrixData {
	result := &MatrixData{}
	// parse first row
	headerRow, err := b.parseFirstRow(rawData[0], len(headerParts))
	if err != nil {
		return nil
	}

	result.columnHeaders = headerRow

	// validate second row
	err = b.validateSecondRow(rawData[1], headerParts)
	if err != nil {
		return nil
	}

	result.columnSubHeaders = headerParts

	// parse rest of rows
	for i := 2; i < len(rawData); i++ {
		headerCol, dataRow, err := b.parseRow(rawData[i])
		if err != nil {
			return nil
		}
		result.rowHeaders = append(result.rowHeaders, headerCol)

		// Split dataRow into parts
		var rowWithParts [][]string
		partSize := len(headerParts)
		for j := 0; j < len(dataRow); j += partSize {
			end := j + partSize
			if end > len(dataRow) {
				end = len(dataRow)
			}
			rowWithParts = append(rowWithParts, dataRow[j:end])
		}
		result.values = append(result.values, rowWithParts)
	}
	return result
}

func (b *MatrixDataBuilder) parseFirstRow(row []string, partCount int) ([]string, error) {
	result := []string{}
	// skip first cell
	for i := 1; i < len(row); i++ {
		if (i-1)%partCount == 0 {
			if row[i] == "" {
				return nil, fmt.Errorf("empty header at: %d", i)
			}
			result = append(result, row[i])
		}
	}
	return result, nil
}

func (b *MatrixDataBuilder) validateSecondRow(row []string, headerParts []string) error {
	// we skip first cell
	for i, cell := range row {
		if i == 0 {
			continue
		}
		partIndex := (i - 1) % len(headerParts)
		if cell != headerParts[partIndex] {
			return fmt.Errorf("invalid second header at: %d. expected: %s, actual: %s", i, headerParts[partIndex], cell)
		}
	}
	return nil
}

func (b *MatrixDataBuilder) parseRow(row []string) (string, []string, error) {
	if len(row) < 2 {
		return "", nil, fmt.Errorf("invalid row")
	}
	return row[0], row[1:], nil
}
