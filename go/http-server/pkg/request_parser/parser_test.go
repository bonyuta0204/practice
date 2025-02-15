package request_parser

import (
	"reflect"
	"strings"
	"testing"

	"github.com/bonyuta0204/practice/go/http-server/pkg/http"
)

func TestParserParse(t *testing.T) {
	tests := []struct {
		name    string
		input   string
		want    http.HttpRequest
		wantErr bool
	}{
		{
			name: "simple GET request",
			input: "GET /index.html HTTP/1.1\r\n" +
				"Host: localhost:8080\r\n" +
				"User-Agent: Mozilla/5.0\r\n" +
				"\r\n",
			want: http.HttpRequest{
				Method:  "GET",
				Path:    "/index.html",
				Headers: map[string]string{
					"Host":       "localhost:8080",
					"User-Agent": "Mozilla/5.0",
				},
				Body: []byte{},
			},
			wantErr: false,
		},
		{
			name: "POST request with body",
			input: "POST /api/users HTTP/1.1\r\n" +
				"Host: localhost:8080\r\n" +
				"Content-Type: application/json\r\n" +
				"Content-Length: 24\r\n" +
				"\r\n" +
				"{\"username\": \"testuser\"}",
			want: http.HttpRequest{
				Method:  "POST",
				Path:    "/api/users",
				Headers: map[string]string{
					"Host":           "localhost:8080",
					"Content-Type":   "application/json",
					"Content-Length": "24",
				},
				Body: []byte("{\"username\": \"testuser\"}"),
			},
			wantErr: false,
		},
		{
			name: "request with multiple headers",
			input: "GET /api/data HTTP/1.1\r\n" +
				"Host: localhost:8080\r\n" +
				"Accept: application/json\r\n" +
				"Authorization: Bearer token123\r\n" +
				"Cache-Control: no-cache\r\n" +
				"\r\n",
			want: http.HttpRequest{
				Method:  "GET",
				Path:    "/api/data",
				Headers: map[string]string{
					"Host":          "localhost:8080",
					"Accept":        "application/json",
					"Authorization": "Bearer token123",
					"Cache-Control": "no-cache",
				},
				Body: []byte{},
			},
			wantErr: false,
		},
		{
			name:    "invalid HTTP version",
			input:   "GET / HTTP/2.0\r\n\r\n",
			wantErr: true,
		},
		{
			name: "missing host header",
			input: "GET / HTTP/1.1\r\n" +
				"Accept: text/html\r\n" +
				"\r\n",
			wantErr: true,
		},
		{
			name: "malformed request line",
			input: "GET\r\n" +
				"Host: localhost:8080\r\n" +
				"\r\n",
			wantErr: true,
		},
		{
			name: "POST without content length",
			input: "POST /api/data HTTP/1.1\r\n" +
				"Host: localhost:8080\r\n" +
				"Content-Type: application/json\r\n" +
				"\r\n" +
				"{\"data\": \"test\"}",
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			reader := strings.NewReader(tt.input)
			parser := New(reader)

			got, err := parser.Parse()
			if (err != nil) != tt.wantErr {
				t.Errorf("Parser.Parse() error = %v, wantErr %v", err, tt.wantErr)
				return
			}

			if !tt.wantErr {
				if got.Method != tt.want.Method {
					t.Errorf("Parser.Parse() Method = %v, want %v", got.Method, tt.want.Method)
				}

				if got.Path != tt.want.Path {
					t.Errorf("Parser.Parse() Path = %v, want %v", got.Path, tt.want.Path)
				}

				if !reflect.DeepEqual(got.Headers, tt.want.Headers) {
					t.Errorf("Parser.Parse() Headers = %v, want %v", got.Headers, tt.want.Headers)
				}

				if !reflect.DeepEqual(got.Body, tt.want.Body) {
					t.Errorf("Parser.Parse() Body = %v, want %v", got.Body, tt.want.Body)
				}
			}
		})
	}
}
