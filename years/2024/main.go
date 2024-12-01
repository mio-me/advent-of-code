package main

import (
	"bufio"
	"log"
	"os"

	"github.com/thorbenbelow/advent-of-code/years/2024/d01"
)

func main() {
	a, err := runPartial("d01/input.txt", d01.A)
	if err != nil {
		log.Fatal(err)
	}
	b, err := runPartial("d01/input.txt", d01.B)
	if err != nil {
		log.Fatal(err)
	}
	log.Printf("Day 01:\nA: %v\nB: %v\n", a, b)
}

func runPartial(path string, fn func(sc *bufio.Scanner) (int, error)) (interface{}, error) {
	file, err := os.Open("d01/input.txt")
	if err != nil {
		return nil, err
	}
	defer file.Close()
	sc := bufio.NewScanner(file)

	res, err := fn(sc)
	if err != nil {
		return nil, err
	}

	return res, nil
}
