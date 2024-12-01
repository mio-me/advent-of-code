package d01

import (
	"bufio"
	"fmt"
	"math"
	"sort"
	"strconv"
	"strings"
)

func A(sc *bufio.Scanner) (int, error) {
	list1 := make([]int, 0)
	list2 := make([]int, 0)

	for sc.Scan() {
		locations := strings.Fields(sc.Text())
		val1, err := strconv.Atoi(locations[0])
		if err != nil {
			return 0, fmt.Errorf("failed to parse value %v in line %v", locations[0], locations)
		}
		val2, err := strconv.Atoi(locations[1])
		if err != nil {
			return 0, fmt.Errorf("failed to parse value %v in line %v", locations[1], locations)
		}

		list1 = append(list1, val1)
		list2 = append(list2, val2)
	}

	sort.Ints(list1)
	sort.Ints(list2)

	res := 0
	for i := range list1 {
		res += int(math.Abs(float64(list1[i] - list2[i])))
	}
	return res, nil
}

func B(sc *bufio.Scanner) (int, error) {
	ids := make([]int, 0)
	m := make(map[int]int)

	for sc.Scan() {
		nums := strings.Fields(sc.Text())
		first, err := strconv.Atoi(nums[0])
		if err != nil {
			return 0, err
		}
		ids = append(ids, first)

		second, err := strconv.Atoi(nums[1])
		if err != nil {
			return 0, err
		}
		m[second] += 1
	}

	var res int
	for _, n := range ids {
		res += n * m[n]
	}

	return res, nil
}
