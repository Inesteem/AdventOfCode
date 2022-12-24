package main

type stringToIntMap struct {
	m map[string]int
}

func (m *stringToIntMap) getIdx(s string) int {
	_, ok := m.m[s]
	if !ok {
		m.m[s] = len(m.m)
	}
	return m.m[s]
}

func newStringToIntMap() stringToIntMap {
	return stringToIntMap{
		m: make(map[string]int, 0),
	}
}
