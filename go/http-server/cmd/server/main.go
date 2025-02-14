package main

import (
	"log"

	"github.com/bonyuta0204/practice/go/http-server/pkg/server"
)

func main() {
	srv := server.New(":8080")
	if err := srv.Start(); err != nil {
		log.Fatal(err)
	}
}
