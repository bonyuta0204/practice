package main

import (
	"fmt"
	"testing"
)

func generateMatrixData(rows, cols int, partsPerCol int) [][]string {
	data := make([][]string, rows+2)
	
	// First row: column headers
	data[0] = make([]string, cols*partsPerCol+1)
	data[0][0] = ""
	for i := 0; i < cols; i++ {
		data[0][i*partsPerCol+1] = fmt.Sprintf("Col%d", i+1)
		for j := 1; j < partsPerCol; j++ {
			data[0][i*partsPerCol+j+1] = ""
		}
	}
	
	// Second row: sub-headers
	data[1] = make([]string, cols*partsPerCol+1)
	data[1][0] = ""
	for i := 0; i < cols; i++ {
		for j := 0; j < partsPerCol; j++ {
			data[1][i*partsPerCol+j+1] = fmt.Sprintf("Part%d", j+1)
		}
	}
	
	// Data rows
	for i := 0; i < rows; i++ {
		data[i+2] = make([]string, cols*partsPerCol+1)
		data[i+2][0] = fmt.Sprintf("Row%d", i+1)
		for j := 0; j < cols*partsPerCol; j++ {
			data[i+2][j+1] = fmt.Sprintf("v%d,%d", i, j)
		}
	}
	
	return data
}

func BenchmarkMatrixDataBuilder_Small(b *testing.B) {
	// 10x10 matrix with 2 parts per column
	rawData := generateMatrixData(10, 10, 2)
	headerParts := []string{"Part1", "Part2"}
	builder := &MatrixDataBuilder{}
	
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		result := builder.Build(rawData, headerParts)
		if result == nil {
			b.Fatal("Build returned nil")
		}
	}
}

func BenchmarkMatrixDataBuilder_Medium(b *testing.B) {
	// 100x100 matrix with 3 parts per column
	rawData := generateMatrixData(100, 100, 3)
	headerParts := []string{"Part1", "Part2", "Part3"}
	builder := &MatrixDataBuilder{}
	
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		result := builder.Build(rawData, headerParts)
		if result == nil {
			b.Fatal("Build returned nil")
		}
	}
}

func BenchmarkMatrixDataBuilder_Large(b *testing.B) {
	// 1000x1000 matrix with 2 parts per column
	rawData := generateMatrixData(1000, 1000, 2)
	headerParts := []string{"Part1", "Part2"}
	builder := &MatrixDataBuilder{}
	
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		result := builder.Build(rawData, headerParts)
		if result == nil {
			b.Fatal("Build returned nil")
		}
	}
}

// Benchmark with varying part counts
func BenchmarkMatrixDataBuilder_VaryingParts(b *testing.B) {
	partCounts := []int{1, 2, 5, 10}
	for _, partCount := range partCounts {
		b.Run(fmt.Sprintf("Parts-%d", partCount), func(b *testing.B) {
			headerParts := make([]string, partCount)
			for i := 0; i < partCount; i++ {
				headerParts[i] = fmt.Sprintf("Part%d", i+1)
			}
			
			rawData := generateMatrixData(50, 50, partCount)
			builder := &MatrixDataBuilder{}
			
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				result := builder.Build(rawData, headerParts)
				if result == nil {
					b.Fatal("Build returned nil")
				}
			}
		})
	}
}

// Benchmark memory allocation
func BenchmarkMatrixDataBuilder_Memory(b *testing.B) {
	rawData := generateMatrixData(100, 100, 3)
	headerParts := []string{"Part1", "Part2", "Part3"}
	builder := &MatrixDataBuilder{}
	
	b.ReportAllocs()
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		result := builder.Build(rawData, headerParts)
		if result == nil {
			b.Fatal("Build returned nil")
		}
	}
}