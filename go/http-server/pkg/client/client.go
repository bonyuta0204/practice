package client

import (
	"fmt"
	"net"
)

// Client represents a TCP client
type Client struct {
	serverAddr string
}

// New creates a new client instance
func New(serverAddr string) *Client {
	return &Client{
		serverAddr: serverAddr,
	}
}

// Send sends a message to the server and returns the response
func (c *Client) Send(message string) (string, error) {
	conn, err := net.Dial("tcp", c.serverAddr)
	if err != nil {
		return "", fmt.Errorf("failed to connect to server: %v", err)
	}
	defer conn.Close()

	// Send the message
	if _, err := fmt.Fprintf(conn, "%s\n", message); err != nil {
		return "", fmt.Errorf("failed to send message: %v", err)
	}

	// Read the response
	response := make([]byte, 1024)
	n, err := conn.Read(response)
	if err != nil {
		return "", fmt.Errorf("failed to read response: %v", err)
	}

	return string(response[:n]), nil
}
