package server

import (
	"fmt"
	"log"
	"net"

	"github.com/bonyuta0204/practice/go/http-server/pkg/request_parser"
)

// Server represents the TCP server
type Server struct {
	port string
}

// New creates a new server instance
func New(port string) *Server {
	return &Server{
		port: port,
	}
}

// Start starts the TCP server
func (s *Server) Start() error {
	ln, err := net.Listen("tcp", s.port)
	if err != nil {
		return err
	}

	fmt.Println("Starting server on port " + s.port)

	for {
		conn, err := ln.Accept()
		if err != nil {
			log.Printf("Error accepting connection: %v\n", err)
			continue
		}
		go s.handleConnection(conn)
	}
}

func (s *Server) handleConnection(conn net.Conn) {
	defer conn.Close()

	p := request_parser.New(conn)
	response, err := p.Parse()

	if err != nil {
		log.Printf("Error parsing response: %v\n", err)
		conn.Write([]byte("broken"))
		return
	}

	log.Printf("Parsed response: %v\n", response)

	conn.Write([]byte("OK"))

}
