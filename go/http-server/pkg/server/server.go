package server

import (
	"bufio"
	"fmt"
	"log"
	"net"
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

	scanner := bufio.NewScanner(conn)
	response := make([]byte, 0, 1024)

	for scanner.Scan() {
		content := scanner.Bytes()
		response = append(response, content...)
		fmt.Println("Received:", string(content))
	}

	if err := scanner.Err(); err != nil {
		log.Printf("Error reading from connection: %v\n", err)
		return
	}

	fmt.Println("Sending response:", string(response))
	if _, err := conn.Write(response); err != nil {
		log.Printf("Error writing response: %v\n", err)
	}
}
