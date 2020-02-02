package app

import (
	"fmt"
	"io"
	"io/ioutil"
	"log"
	"net/http"
)

type App struct {
}

func NewApp() *App {
	a := App{}
	return &a
}

func (a *App) Run() error {
	log.Println("Ocean started")

	appendHandler := func(writer http.ResponseWriter, request *http.Request) {
		body, err := ioutil.ReadAll(request.Body)

		if err != nil {
			http.Error(writer, "can't read body", http.StatusBadRequest)
			log.Println("Error reading body: " + err.Error())
		}

		fmt.Println(string(body))
		writer.Header().Set("Access-Control-Allow-Origin", "*")
		writer.Header().Set("Access-Control-Allow-Headers", "*")
		io.WriteString(writer, "OK\n")
	}

	http.HandleFunc("/append", appendHandler)
	return http.ListenAndServe(":11000", nil)
}
