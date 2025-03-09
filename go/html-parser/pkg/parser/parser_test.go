package parser

import (
	"strings"
	"testing"
)

func TestParse(t *testing.T) {

	html := strings.NewReader("<div>")

	parser := New(html)

	result, err := parser.Parse()

	if err != nil {
		t.Fatalf("Error %s", err)
	}

	if result.tag != "div" {
		t.Errorf("Expecxted div but found %v", result)
	}

}

func TestParseElement(t *testing.T) {

	element := strings.NewReader("Test\n<div>")

	parser := New(element)

	el, _ := parser.parseElement()

	if el.content != "Test\n" {
		t.Fatalf("content is not correct. %s", el.content)
	}
}

func TestParseNode(t *testing.T) {

	t.Run("shallow div", func(t *testing.T) {
		element := strings.NewReader("<div>test</div>")
		parser := New(element)
		got, err := parser.parseNode()

		if err != nil {
			t.Fatalf("Error happend. %v", err)
		}

		if got.tag != "div" {
			t.Fatalf("Unmatched tag")
		}

		textNode := got.children[0]

		if textNode.content != "test" {
			t.Fatalf("Expecxted 'test' but found %s", textNode.content)
		}
	})
}
