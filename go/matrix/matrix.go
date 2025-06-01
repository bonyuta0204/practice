package main

import (
	"fmt"
	"runtime"
	"sync"
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
	partSize := len(headerParts)
	// Pre-allocate memory for slices
	numRows := len(rawData) - 2
	result.rowHeaders = make([]string, 0, numRows)
	result.values = make([][][]string, 0, numRows)
	
	for i := 2; i < len(rawData); i++ {
		headerCol, rowWithParts, err := b.parseRow(rawData[i], partSize)
		if err != nil {
			return nil
		}
		result.rowHeaders = append(result.rowHeaders, headerCol)
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

func (b *MatrixDataBuilder) parseRow(row []string, partSize int) (string, [][]string, error) {
	if len(row) < 2 {
		return "", nil, fmt.Errorf("invalid row")
	}
	
	header := row[0]
	data := row[1:]
	
	// Split data into parts
	var parts [][]string
	for i := 0; i < len(data); i += partSize {
		end := i + partSize
		if end > len(data) {
			end = len(data)
		}
		parts = append(parts, data[i:end])
	}
	
	return header, parts, nil
}

type rowResult struct {
	index      int
	header     string
	data       [][]string
	err        error
}

func (b *MatrixDataBuilder) BuildParallel(rawData [][]string, headerParts []string) *MatrixData {
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

	// Process data rows in parallel
	numRows := len(rawData) - 2
	if numRows <= 0 {
		return result
	}

	// For small matrices, use sequential processing
	if numRows < 100 {
		return b.Build(rawData, headerParts)
	}

	// Determine number of workers based on data size
	numWorkers := runtime.NumCPU()
	if numRows < numWorkers*10 {
		numWorkers = numRows / 10
		if numWorkers < 1 {
			numWorkers = 1
		}
	}

	// Pre-allocate result slices
	result.rowHeaders = make([]string, numRows)
	result.values = make([][][]string, numRows)

	// Create channels for work distribution
	type job struct {
		start, end int
	}
	jobs := make(chan job, numWorkers)
	errors := make(chan error, numWorkers)

	// Start workers
	var wg sync.WaitGroup
	partSize := len(headerParts)
	batchSize := numRows / numWorkers
	if batchSize < 1 {
		batchSize = 1
	}
	
	for w := 0; w < numWorkers; w++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for j := range jobs {
				for idx := j.start; idx < j.end && idx < numRows; idx++ {
					i := idx + 2 // actual row index in rawData
					headerCol, rowWithParts, err := b.parseRow(rawData[i], partSize)
					if err != nil {
						errors <- err
						return
					}
					
					result.rowHeaders[idx] = headerCol
					result.values[idx] = rowWithParts
				}
			}
		}()
	}

	// Send jobs in batches
	for i := 0; i < numRows; i += batchSize {
		end := i + batchSize
		if end > numRows {
			end = numRows
		}
		jobs <- job{start: i, end: end}
	}
	close(jobs)

	// Wait for completion
	wg.Wait()
	close(errors)

	// Check for errors
	if err := <-errors; err != nil {
		return nil
	}

	return result
}
