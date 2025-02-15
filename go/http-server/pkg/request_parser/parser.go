package request_parser

import (
	"bufio"
	"errors"
	"io"
	"strconv"
	"strings"

	"github.com/bonyuta0204/practice/go/http-server/pkg/http"
)

type Parser struct {
	reader *bufio.Reader
}

func New(r io.Reader) *Parser {
	return &Parser{
		reader: bufio.NewReader(r),
	}
}

func (p *Parser) Parse() (http.HttpRequest, error) {
	// Initialize request with empty body
	request := http.HttpRequest{
		Body: []byte{},
	}

	// Parse request line
	requestLine, err := p.reader.ReadString('\n')
	if err != nil {
		return request, err
	}
	requestLine = strings.TrimSuffix(requestLine, "\r\n")
	method, path, version, err := p.parseRequestLine(requestLine)
	if err != nil {
		return request, err
	}

	request.Method = method
	request.Path = path

	// Verify HTTP version
	if version != "HTTP/1.1" {
		return request, errors.New("unsupported HTTP version")
	}

	// Parse headers
	headers, err := p.parseHeaders()
	if err != nil {
		return request, err
	}
	request.Headers = headers

	// Verify required headers for HTTP/1.1
	if _, exists := headers["Host"]; !exists {
		return request, errors.New("missing required Host header")
	}

	// Parse body if present
	if contentLengthStr, exists := headers["Content-Length"]; exists {
		contentLength, err := strconv.Atoi(contentLengthStr)
		if err != nil {
			return request, errors.New("invalid Content-Length")
		}
		if contentLength > 0 {
			body := make([]byte, contentLength)
			n, err := io.ReadFull(p.reader, body)
			if err != nil {
				return request, err
			}
			if n != contentLength {
				return request, errors.New("incomplete body")
			}
			request.Body = body
		}
	} else if method == "POST" {
		return request, errors.New("Content-Length required for POST requests")
	}

	return request, nil
}

func (p *Parser) parseRequestLine(line string) (method string, path string, version string, err error) {
	parts := strings.Split(line, " ")
	if len(parts) != 3 {
		return "", "", "", errors.New("malformed request line")
	}
	return parts[0], parts[1], parts[2], nil
}

func (p *Parser) parseHeaders() (map[string]string, error) {
	headers := make(map[string]string)
	for {
		line, err := p.reader.ReadString('\n')
		if err != nil {
			return nil, err
		}
		line = strings.TrimSuffix(line, "\r\n")
		
		// Empty line marks the end of headers
		if line == "" {
			break
		}

		// Split header line into key and value
		parts := strings.SplitN(line, ":", 2)
		if len(parts) != 2 {
			return nil, errors.New("malformed header")
		}

		key := strings.TrimSpace(parts[0])
		value := strings.TrimSpace(parts[1])
		headers[key] = value
	}
	return headers, nil
}
