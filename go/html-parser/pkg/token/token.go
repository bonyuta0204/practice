package token

type TokenType string

type Token struct {
	Type    TokenType
	Literal string
}

const (
	OPEN_TAG_START  = "OPEN_TAG_START"
	TAG_NAME        = "TAG_NAME"
	TAG_END         = "TAG_END"
	CLOSE_TAG_START = "CLOSE_TAG_START"
	TEXT            = "TEXT"
)
