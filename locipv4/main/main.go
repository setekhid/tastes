package main

import (
	"fmt"
	"github.com/setekhid/locipv4"
	"math/rand"
	"os"
)

func main() {
	db_name := os.Args[1]

	db := &locipv4.V4db{}
	db.Load(db_name)

	for i := 0; i < 16; i++ {
		ipv4 := rand.Uint32()
		loc := db.Locate(ipv4)
		fmt.Println("ipv4: %d.%d.%d.%d => country: %s\n", byte(ipv4>>24), byte(ipv4>>16), byte(ipv4>>8), byte(ipv4), loc.Country)
	}
}
