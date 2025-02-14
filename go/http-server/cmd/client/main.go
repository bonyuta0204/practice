package main

import (
	"bufio"
	"fmt"
	"log"
	"os"

	"github.com/bonyuta0204/practice/go/http-server/pkg/client"
)

func main() {
	c := client.New(":8080")
	scanner := bufio.NewScanner(os.Stdin)

	fmt.Println("Enter messages to send to server (Ctrl+D to exit):")
	for scanner.Scan() {
		message := scanner.Text()
		response, err := c.Send(message)
		if err != nil {
			log.Printf("Error: %v\n", err)
			continue
		}
		fmt.Printf("Server response: %s\n", response)
	}
}
