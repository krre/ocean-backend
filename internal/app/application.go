package app

import (
	"errors"
	"log"
	"net/http"

	"github.com/krre/ocean-backend/internal/pkg/config"
	"github.com/krre/ocean-backend/internal/pkg/handler"
)

type App struct {
	Config *config.Config
}

func NewApp() (*App, error) {
	a := App{}
	conf, err := config.NewConfig()

	if err != nil {
		return nil, errors.New("failed to create config: " + err.Error())
	}

	a.Config = conf

	return &a, nil
}

func (a *App) Run() error {
	http.HandleFunc("/append", handler.Append)
	log.Println("Ocean started")

	return http.ListenAndServe(":11000", nil)
}
