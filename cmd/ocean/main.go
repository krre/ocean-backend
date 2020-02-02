package main

import (
	"fmt"
	"io"
	"io/ioutil"
	"log"
	"net/http"
)

func main() {
	fmt.Println("Ocean started")

	appendHandler := func(writer http.ResponseWriter, request *http.Request) {
		body, err := ioutil.ReadAll(request.Body)

		if err != nil {
			fmt.Errorf("Error reading body: %v", err)
			http.Error(writer, "can't read body", http.StatusBadRequest)
			return
		}

		fmt.Println(string(body))
		writer.Header().Set("Access-Control-Allow-Origin", "*")
		writer.Header().Set("Access-Control-Allow-Headers", "*")
		io.WriteString(writer, "OK\n")
	}

	http.HandleFunc("/append", appendHandler)
	log.Fatal(http.ListenAndServe(":11000", nil))
}
