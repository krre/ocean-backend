package main

import (
	"fmt"
	"io"
	"log"
	"net/http"
)

func main() {
	fmt.Println("Ocean started")

	appendHandler := func(w http.ResponseWriter, req *http.Request) {
		w.Header().Set("Access-Control-Allow-Origin", "*")
		io.WriteString(w, "OK\n")
		fmt.Println("appendHandler")
	}

	http.HandleFunc("/append", appendHandler)
	log.Fatal(http.ListenAndServe(":11000", nil))
}
