package main

import (
	"log"

	"github.com/julieqiu/gcloud/internal/gcloud"
)

func main() {
	if err := gcloud.Run(); err != nil {
		log.Fatal(err)
	}
}
