package http

import (
	"fmt"
	"strconv"
)

// Standard HTTP status codes
var statusMap = map[int]string{
	200: "OK",
	400: "Bad Request",
	404: "Not Found",
	500: "Internal Server Error",
}

// Helper function to get status text
func getStatusText(code int) string {
	if text, exists := statusMap[code]; exists {
		return text
	}
	return "Unknown Status"
}

type HttpRequest struct {
	Method  string
	Path    string
	Headers map[string]string
	Body    []byte
}

type HttpResponse struct {
	StatusCode int
	Headers    map[string]string
	Body       []byte
}

func (r *HttpResponse) ToByte() []byte {
	fmt.Printf("StatusCode: %d\n", r.StatusCode)
	reason := getStatusText(r.StatusCode)

	var lines []string
	lines = append(lines, fmt.Sprintf("HTTP/1.1 %d %s", r.StatusCode, reason))

	for k, v := range r.Headers {
		lines = append(lines, fmt.Sprintf("%s: %s", k, v))
	}

	// Blank line before body
	lines = append(lines, "")

	var data []byte
	for _, line := range lines {
		data = append(data, []byte(line+"\r\n")...)
	}

	if len(r.Body) > 0 {
		data = append(data, r.Body...)
	}

	return data
}

func (r *HttpResponse) SetTextBody(s string) {
	if r.Headers == nil {
		r.Headers = make(map[string]string)
	}

	body := []byte(s)
	r.Headers["Content-Length"] = strconv.Itoa(len(body))
	r.Headers["Content-Type"] = "text/html"
	r.Body = body
}

func NewResponse(statusCode int, headers map[string]string) *HttpResponse {
	if headers == nil {
		headers = make(map[string]string)
	}

	return &HttpResponse{
		StatusCode: statusCode,
		Headers:    headers,
		Body:       []byte{},
	}
}

