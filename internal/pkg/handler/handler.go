package handler

import (
	"io"
	"io/ioutil"
	"log"
	"net/http"
)

func Append(writer http.ResponseWriter, request *http.Request) {
	body, err := ioutil.ReadAll(request.Body)

	if err != nil {
		http.Error(writer, "can't read body", http.StatusBadRequest)
		log.Println("Error reading body: " + err.Error())
	}

	log.Println(string(body))

	writer.Header().Set("Access-Control-Allow-Origin", "*")
	writer.Header().Set("Access-Control-Allow-Headers", "*")
	io.WriteString(writer, "OK\n")
}
