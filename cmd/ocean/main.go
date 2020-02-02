package main

import (
	"log"

	"github.com/krre/ocean-backend/internal/app"
)

func main() {
	a := app.NewApp()
	if err := a.Run(); err != nil {
		log.Fatalf("Error starting server: %v", err)
	}
}
