package parser

import (
	"reflect"
	"strings"
	"testing"
)

func TestParserParse(t *testing.T) {
	httpResponse := "HTTP/1.1 200 OK\r\n" +
		"Content-Type: text/plain\r\n" +
		"Content-Length: 13\r\n" +
		"\r\n" +
		"Hello, World!"

	reader := strings.NewReader(httpResponse)
	parser := New(reader)

	response, err := parser.Parse()
	if err != nil {
		t.Errorf("Failed to parse: %v", err)
	}

	if response.StatusCode != 200 {
		t.Errorf("Expected status code 200, got %d", response.StatusCode)
	}

	expectedHeaders := map[string]string{
		"Content-Type":   "text/plain",
		"Content-Length": "13",
	}

	if !reflect.DeepEqual(response.Headers, expectedHeaders) {
		t.Errorf("Expected headers %v, got %v", expectedHeaders, response.Headers)
	}

	if string(response.Body) != "Hello, World!" {
		t.Errorf("Expected body \"Hello, World!\", got %s", string(response.Body))
	}
}
