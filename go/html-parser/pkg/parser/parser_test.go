package parser

import (
	"strings"
	"testing"
)

func TestParse(t *testing.T) {
	t.Run("simple div", func(t *testing.T) {
		html := strings.NewReader("<div>a</div>")
		parser := New(html)
		result, err := parser.Parse()
		if err != nil {
			t.Fatalf("Error %s", err)
		}
		if result.tag != "div" {
			t.Errorf("Expected div but found %v", result)
		}
	})

	t.Run("complex nested structure", func(t *testing.T) {
		html := strings.NewReader("<html><body><div>Hello</div></body></html>")
		parser := New(html)
		result, err := parser.Parse()
		if err != nil {
			t.Fatalf("Error %s", err)
		}
		if result.tag != "html" {
			t.Errorf("Expected html but found %s", result.tag)
		}
		if len(result.children) != 1 {
			t.Fatalf("Expected 1 child but found %d", len(result.children))
		}
		if result.children[0].tag != "body" {
			t.Errorf("Expected body but found %s", result.children[0].tag)
		}
		if len(result.children[0].children) != 1 {
			t.Fatalf("Expected 1 child but found %d", len(result.children[0].children))
		}
		if result.children[0].children[0].tag != "div" {
			t.Errorf("Expected div but found %s", result.children[0].children[0].tag)
		}
		textNode := result.children[0].children[0].children[0]
		if textNode.content != "Hello" {
			t.Errorf("Expected Hello but found %s", textNode.content)
		}
	})
}

func TestParseElement(t *testing.T) {
	t.Run("simple text", func(t *testing.T) {
		element := strings.NewReader("Test\n<div>")
		parser := New(element)
		el, _ := parser.parseElement()
		if el.content != "Test\n" {
			t.Fatalf("content is not correct. %s", el.content)
		}
	})

	t.Run("tag without text", func(t *testing.T) {
		element := strings.NewReader("<div></div>")
		parser := New(element)
		el, err := parser.parseElement()
		if err != nil {
			t.Fatalf("Error occurred: %v", err)
		}
		if el.tag != "div" {
			t.Errorf("Expected div but found %s", el.tag)
		}
		if len(el.children) != 0 {
			t.Errorf("Expected 0 children but found %d", len(el.children))
		}
	})
}

func TestParseNode(t *testing.T) {
	t.Run("shallow div", func(t *testing.T) {
		element := strings.NewReader("<div>test</div>")
		parser := New(element)
		got, err := parser.parseNode()
		if err != nil {
			t.Fatalf("Error happened. %v", err)
		}
		if got.tag != "div" {
			t.Fatalf("Unmatched tag")
		}
		textNode := got.children[0]
		if textNode.content != "test" {
			t.Fatalf("Expected 'test' but found %s", textNode.content)
		}
	})

	t.Run("nested tags", func(t *testing.T) {
		element := strings.NewReader("<div><span>hello</span></div>")
		parser := New(element)
		got, err := parser.parseNode()
		if err != nil {
			t.Fatalf("Error happened. %v", err)
		}
		if got.tag != "div" {
			t.Fatalf("Expected div but found %s", got.tag)
		}
		if len(got.children) != 1 {
			t.Fatalf("Expected 1 child but found %d", len(got.children))
		}
		spanNode := got.children[0]
		if spanNode.tag != "span" {
			t.Fatalf("Expected span but found %s", spanNode.tag)
		}
		textNode := spanNode.children[0]
		if textNode.content != "hello" {
			t.Fatalf("Expected 'hello' but found %s", textNode.content)
		}
	})

	t.Run("multiple children", func(t *testing.T) {
		element := strings.NewReader("<div>text1<p>paragraph</p>text2</div>")
		parser := New(element)
		got, err := parser.parseNode()
		if err != nil {
			t.Fatalf("Error happened. %v", err)
		}
		if got.tag != "div" {
			t.Fatalf("Expected div but found %s", got.tag)
		}
		if len(got.children) != 3 {
			t.Fatalf("Expected 3 children but found %d", len(got.children))
		}
		if got.children[0].tag != "text" || got.children[0].content != "text1" {
			t.Errorf("First child incorrect: %s - %s", got.children[0].tag, got.children[0].content)
		}
		if got.children[1].tag != "p" {
			t.Errorf("Second child incorrect: %s", got.children[1].tag)
		}
		if got.children[2].tag != "text" || got.children[2].content != "text2" {
			t.Errorf("Third child incorrect: %s - %s", got.children[2].tag, got.children[2].content)
		}
	})

	t.Run("mixed content with whitespace", func(t *testing.T) {
		element := strings.NewReader("<div>\n  <p>Hello</p>\n  <p>World</p>\n</div>")
		parser := New(element)
		got, err := parser.parseNode()
		if err != nil {
			t.Fatalf("Error happened. %v", err)
		}
		if got.tag != "div" {
			t.Fatalf("Expected div but found %s", got.tag)
		}
		if len(got.children) != 5 {
			t.Fatalf("Expected 5 children but found %d", len(got.children))
		}
		// Check text nodes contain expected whitespace
		if got.children[0].tag != "text" || got.children[0].content != "\n  " {
			t.Errorf("Whitespace node incorrect: %s", got.children[0].content)
		}
		// Check paragraph nodes
		if got.children[1].tag != "p" || got.children[1].children[0].content != "Hello" {
			t.Errorf("First paragraph incorrect")
		}
		if got.children[3].tag != "p" || got.children[3].children[0].content != "World" {
			t.Errorf("Second paragraph incorrect")
		}
	})
}

func TestEdgeCases(t *testing.T) {
	t.Run("empty string", func(t *testing.T) {
		element := strings.NewReader("")
		parser := New(element)
		_, err := parser.Parse()
		if err == nil {
			t.Fatalf("Expected error for empty input but got none")
		}
	})

	t.Run("text only", func(t *testing.T) {
		element := strings.NewReader("Just text, no HTML")
		parser := New(element)
		_, err := parser.Parse()
		if err == nil {
			t.Fatalf("Expected parse error but no error happened: %v", err)
		}
	})

	t.Run("unclosed tag", func(t *testing.T) {
		element := strings.NewReader("<div>text")
		parser := New(element)
		_, err := parser.Parse()
		if err == nil {
			t.Fatalf("Expected error for unclosed tag but got none")
		}
	})

	t.Run("mismatched tags", func(t *testing.T) {
		element := strings.NewReader("<div>text</span>")
		parser := New(element)
		_, err := parser.Parse()
		if err == nil {
			t.Fatalf("Expected error for mismatched tags but got none")
		}
	})
}
