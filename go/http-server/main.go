package main

import (
	"bufio"
	"fmt"
	"log"
	"net"
)

func main() {

	// Define the port
	port := ":8080"
	ln, err := net.Listen("tcp", port)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Starting server on port " + port)

	for {
		conn, err := ln.Accept()
		if err != nil {
			log.Fatal(err)
		}
		go handleConnection(conn)
	}

}

func handleConnection(conn net.Conn) {
	defer conn.Close()

	scanner := bufio.NewScanner(conn)

	response := make([]byte, 1024)

	for scanner.Scan() {
		content := scanner.Bytes()
		response = append(response, content...)
		fmt.Println(string(content))

	}

  fmt.Println("response" ,string(response))
	conn.Write(response)

}
