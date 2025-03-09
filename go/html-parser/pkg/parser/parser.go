package parser

import (
	"bufio"
	"fmt"
	"io"
)

type Node struct {
	parent   *Node
	tag      string
	content  string
	children []*Node
}

type Parser struct {
	reader *bufio.Reader
}

func New(reader io.Reader) *Parser {
	return &Parser{
		reader: bufio.NewReader(reader),
	}
}

func (p *Parser) Parse() (*Node, error) {

	return p.parseNode()
}

func (p *Parser) parseElement() (*Node, error) {

	ret, err := p.parseTextNode()

	if err != nil {
		return nil, err
	}

	if ret != nil {
		return ret, nil
	}

	ret, err = p.parseNode()

	return ret, err

}

func (p *Parser) parseTextNode() (*Node, error) {
	s, err := p.consumeUntil('<')

	if err != nil {
		return nil, err
	}

	if len(s) == 0 {
		return nil, nil
	}

	return &Node{
		tag:     "text",
		content: s,
	}, nil
}

func (p *Parser) parseNode() (*Node, error) {

	t, err := p.reader.Peek(1)

	if err == io.EOF {
		return nil, fmt.Errorf("Expected HTML tag but not found")
	} else if err != nil {
		return nil, err
	}

	if rune(t[0]) != '<' {
		return nil, fmt.Errorf("Expected HTML tag but not found")
	}

	tag, err := p.consumeOpenTag()

	if err != nil {
		return nil, err
	}

	children := make([]*Node, 0)

	var el *Node

	for {
		// see if next is closed tag
		n, err := p.reader.Peek(2)

		if err != nil {
			return nil, err
		}

		if string(n) == "</" {
			_, err = p.consumeCloseTag(tag)

			if err != nil {
				return nil, err
			}

			return &Node{
				tag:      tag,
				content:  "",
				children: children,
			}, nil
		}

		el, err = p.parseElement()

		if err != nil {
			return nil, err
		}

		if el == nil {
			break
		}

		children = append(children, el)

	}

	return &Node{
		tag:      tag,
		content:  "",
		children: children,
	}, nil
}

func (p *Parser) consumeUntil(r rune) (string, error) {
	var ret []rune

	for {
		t, _, err := p.reader.ReadRune()

		if err == io.EOF {
			return string(ret), nil
		} else if err != nil {
			return "", err
		}

		if t == r {
			p.reader.UnreadRune()
			return string(ret), nil
		}
		ret = append(ret, t)
	}
}

func (p *Parser) consumeOpenTag() (string, error) {
	t, _, err := p.reader.ReadRune()
	ret := make([]rune, 0)

	if err != nil {
		return "", err
	}

	if t != '<' {
		return "", fmt.Errorf("Expected < but found %c", t)
	}

	for {
		t, _, err = p.reader.ReadRune()
		if err != nil {
			return "", err
		}

		if t == '>' {
			return string(ret), nil
		}

		ret = append(ret, t)
	}
}

func (p *Parser) consumeCloseTag(tag string) (string, error) {
	closeTag := "</" + tag + ">"

	for _, r := range closeTag {

		c, _, err := p.reader.ReadRune()

		if err != nil {
			return "", err
		}

		if c != r {
			return "", fmt.Errorf("Expected %c but found %c", r, c)
		}
	}

	return closeTag, nil
}
