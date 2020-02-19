package main

import (
	"log"

	"github.com/krre/ocean-backend/internal/app"
)

func main() {
	a, err := app.NewApp()

	if err != nil {
		log.Fatalf("Error creating application: %v", err)
	}

	if err := a.Run(); err != nil {
		log.Fatalf("Error starting server: %v", err)
	}
}
