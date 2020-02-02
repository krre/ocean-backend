package app

import (
	"log"
	"net/http"

	"github.com/krre/ocean-backend/internal/pkg/handler"
)

type App struct {
}

func NewApp() *App {
	a := App{}
	return &a
}

func (a *App) Run() error {
	http.HandleFunc("/append", handler.Append)
	log.Println("Ocean started")

	return http.ListenAndServe(":11000", nil)
}
