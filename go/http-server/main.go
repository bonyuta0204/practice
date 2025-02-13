package main

import (
	"fmt"
	"log"
	"net/http"
)

// Handler function for the root endpoint
func homeHandler(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintln(w, "Hello, World! Welcome to my Go HTTP server.")
}

func main() {
	// Register the handler
	http.HandleFunc("/", homeHandler)

	// Define the port
	port := "8080"
	fmt.Println("Starting server on port " + port)

	// Start the server
	err := http.ListenAndServe(":"+port, nil)
	if err != nil {
		log.Fatal("Server failed to start:", err)
	}
}

