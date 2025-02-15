package parser

import (
	"bufio"
	"errors"
	"io"
	"strconv"

	"github.com/bonyuta0204/practice/go/http-server/pkg/http"
)

type Parser struct {
	reader *bufio.Reader
}

func (p *Parser) Parse() (http.HttpResponse, error) {
	_, err := p.consumeToken("HTTP/1.1")
	if err != nil {
		return http.HttpResponse{}, errors.New("Failed to parse HTTP")
	}
	p.skipSpaces()

	statusCode, err := p.consumeInt()
	if err != nil {
		return http.HttpResponse{}, err
	}

	p.skipSpaces()
	_, err = p.consumeToken("OK")
	if err != nil {
		return http.HttpResponse{}, err
	}

	_, err = p.consumeToken("\r\n")
	if err != nil {
		return http.HttpResponse{}, err
	}

	headers, err := p.parseHeaders()
	if err != nil {
		return http.HttpResponse{}, err
	}

	contentLength, err := strconv.Atoi(headers["Content-Length"])
	if err != nil {
		return http.HttpResponse{}, err
	}

	buf := make([]byte, int(contentLength))

	_, err = p.reader.Read(buf)
	if err != nil {
		return http.HttpResponse{}, err
	}

	return http.HttpResponse{
		StatusCode: statusCode,
		Headers:    headers,
		Body:       buf,
	}, nil
}

func (p *Parser) consumeToken(token string) (string, error) {
	for _, r := range token {
		c, _, err := p.reader.ReadRune()

		if err != nil {
			return "", err
		} else if c == r {
			continue
		} else {
			return "", errors.New("Cannot consume")
		}
	}

	return token, nil
}

func (p *Parser) skipSpaces() {
	for {
		c, err := p.reader.Peek(1)
		if err != nil {
			return
		}
		if c[0] == byte(' ') {
			p.reader.ReadByte()
		} else {
			return
		}
	}
}

func (p *Parser) consumeInt() (int, error) {
	var result int
	var hasDigit bool

	for {
		c, err := p.reader.Peek(1)
		if err != nil {
			if err == io.EOF && hasDigit {
				return result, nil
			}
			return 0, err
		}

		// Check if the character is a digit
		if c[0] >= '0' && c[0] <= '9' {
			hasDigit = true
			digit := int(c[0] - '0')
			result = result*10 + digit
			p.reader.ReadByte() // Consume the digit
		} else {
			if !hasDigit {
				return 0, errors.New("no digits found")
			}
			return result, nil
		}
	}
}

func (p *Parser) parseHeaders() (map[string]string, error) {
	headers := make(map[string]string)

	for {
		// Check if we've reached the end of headers (empty line)
		c, err := p.reader.Peek(2)
		if err != nil {
			return nil, err
		}
		if string(c) == "\r\n" {
			p.reader.ReadByte() // consume \r
			p.reader.ReadByte() // consume \n
			break
		}

		// Parse header line
		key, value, err := p.parseHeaderLine()
		if err != nil {
			return nil, err
		}
		headers[key] = value
	}

	return headers, nil
}

func (p *Parser) parseHeaderLine() (string, string, error) {
	// Read until ':'
	key := ""
	for {
		c, err := p.reader.Peek(1)
		if err != nil {
			return "", "", err
		}

		if c[0] == ':' {
			p.reader.ReadByte() // consume ':'
			break
		}

		p.reader.ReadByte() // consume the character
		key += string(c[0])
	}

	p.skipSpaces()

	// Read until \r\n
	value := ""
	for {
		c, err := p.reader.Peek(1)
		if err != nil {
			return "", "", err
		}

		if c[0] == '\r' {
			p.reader.ReadByte() // consume \r
			c, err := p.reader.Peek(1)
			if err != nil {
				return "", "", err
			}
			if c[0] == '\n' {
				p.reader.ReadByte() // consume \n
				break
			}
		}

		p.reader.ReadByte() // consume the character
		value += string(c[0])
	}

	return key, value, nil
}

func New(r io.Reader) *Parser {
	return &Parser{reader: bufio.NewReader(r)}
}
