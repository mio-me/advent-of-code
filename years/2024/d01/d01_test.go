package d01

import (
	"bufio"
	"os"
	"testing"
)

func TestA(t *testing.T) {
	file, err := os.Open("./test_input.txt")
	if err != nil {
		t.Fatal(err)
	}
	defer file.Close()
	sc := bufio.NewScanner(file)

	correctAnswer := 11

	res, err := A(sc)
	if err != nil {
		t.Fatal(err)
	}

	if res != correctAnswer {
		t.Fatalf("wrong answer: got %v but expected %v", res, correctAnswer)
	}
}

func TestB(t *testing.T) {
	file, err := os.Open("./test_input.txt")
	if err != nil {
		t.Fatal(err)
	}
	defer file.Close()
	sc := bufio.NewScanner(file)

	correctAnswer := 31

	res, err := B(sc)
	if err != nil {
		t.Fatal(err)
	}

	if res != correctAnswer {
		t.Fatalf("wrong answer: got %v but expected %v", res, correctAnswer)
	}
}
