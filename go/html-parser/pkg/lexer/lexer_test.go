package lexer

import (
	"html-parser/pkg/token"
	"testing"
)

type LexerTestCase struct {
	name  string
	input string
	want  []token.Token
}

func TestDocument(t *testing.T) {
	testCases := []LexerTestCase{
		{
			name:  "openTag",
			input: "<div>",
			want: []token.Token{
				{Type: token.OPEN_TAG_START, Literal: "<"},
				{Type: token.TAG_NAME, Literal: "div"},
				{Type: token.TAG_END, Literal: ">"},
			},
		},
		{
			name:  "open and close tag",
			input: "<div></div>",
			want: []token.Token{
				{Type: token.OPEN_TAG_START, Literal: "<"},
				{Type: token.TAG_NAME, Literal: "div"},
				{Type: token.TAG_END, Literal: ">"},
				{Type: token.CLOSE_TAG_START, Literal: "</"},
				{Type: token.TAG_NAME, Literal: "div"},
				{Type: token.TAG_END, Literal: ">"},
			},
		},
		{
			name:  "nested tag with text",
			input: "<div>content1\n<p>content2</p>content3</div>",
			want: []token.Token{
				{Type: token.OPEN_TAG_START, Literal: "<"},
				{Type: token.TAG_NAME, Literal: "div"},
				{Type: token.TAG_END, Literal: ">"},
				{Type: token.TEXT, Literal: "content1\n"},
				{Type: token.OPEN_TAG_START, Literal: "<"},
				{Type: token.TAG_NAME, Literal: "p"},
				{Type: token.TAG_END, Literal: ">"},
				{Type: token.TEXT, Literal: "content2"},
				{Type: token.CLOSE_TAG_START, Literal: "</"},
				{Type: token.TAG_NAME, Literal: "p"},
				{Type: token.TAG_END, Literal: ">"},
				{Type: token.TEXT, Literal: "content3"},
				{Type: token.CLOSE_TAG_START, Literal: "</"},
				{Type: token.TAG_NAME, Literal: "div"},
				{Type: token.TAG_END, Literal: ">"},
			},
		},
	}

	for _, tt := range testCases {
		t.Run(tt.name, func(t *testing.T) {
			lexer := New(tt.input)
			err := lexer.Run()

			if err != nil {
				t.Fatalf("Failed to run lexer: %s", err)
			}

			if len(lexer.Tokens()) != len(tt.want) {
				t.Fatalf("Expected %d tokens but found %d tokens", len(tt.want), len(lexer.Tokens()))
			}

			for i, tk := range lexer.Tokens() {
				if tk.Type != tt.want[i].Type {
					t.Errorf("Token[%d] - Type mismatch: expected '%s', got '%s'", i, tt.want[i].Type, tk.Type)
				} else if tk.Literal != tt.want[i].Literal {
					t.Errorf("Token[%d] - Literal mismatch: expected '%s', got '%s'", i, tt.want[i].Literal, tk.Literal)
				}
			}
		})
	}
}
