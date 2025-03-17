package lexer

import (
	"html-parser/pkg/token"
	"strings"
)

type Lexer struct {
	input  string
	pos    int
	tokens []token.Token
}

func New(input string) Lexer {
	return Lexer{input: input, pos: 0}
}

func (l *Lexer) Tokens() []token.Token {
	return l.tokens
}

func (l *Lexer) Run() error {
	for l.pos < len(l.input) {
		if l.peekMulti(2) == "</" {
			l.pos += 2
			l.appendToken(token.CLOSE_TAG_START, "</")
			tagName, err := l.consumeTagName()

			if err != nil {
				return err
			}
			l.appendToken(token.TAG_NAME, tagName)
			continue
		}

		if l.peek() == '<' {
			l.pos += 1
			l.appendToken(token.OPEN_TAG_START, "<")
			tagName, err := l.consumeTagName()

			if err != nil {
				return err
			}
			l.appendToken(token.TAG_NAME, tagName)

			continue
		}

		if l.peek() == '>' {
			l.pos += 1
			l.appendToken(token.TAG_END, ">")
			continue
		}

		s, err := l.consumeText()

		if err != nil {
			return err
		}

		if len(s) > 0 {
			l.appendToken(token.TEXT, s)
		} else {
			// avoid infinite loop
			l.pos++
		}

	}
	return nil

}

func (l *Lexer) peek() rune {
	return rune(l.input[l.pos])
}

func (l *Lexer) peekMulti(count int) string {
	end := min(l.pos+count, len(l.input))
	return l.input[l.pos:end]
}

func (l *Lexer) appendToken(tokenType token.TokenType, literal string) {
	l.tokens = append(l.tokens, token.Token{Type: tokenType, Literal: literal})
}

func (l *Lexer) consumeTagName() (string, error) {

	var sb strings.Builder

	for l.pos < len(l.input) && l.peek() != '>' {
		_, err := sb.WriteRune(l.peek())
		if err != nil {
			return "", err
		}
		l.pos++

	}
	return sb.String(), nil
}

func (l *Lexer) consumeText() (string, error) {

	var sb strings.Builder

	for l.pos < len(l.input) && l.peek() != '>' && l.peek() != '<' {
		_, err := sb.WriteRune(l.peek())
		if err != nil {
			return "", err
		}
		l.pos++

	}
	return sb.String(), nil
}
