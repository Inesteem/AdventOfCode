package main

import (
	"fmt"
	"log"
)

// Disclaimer: most funcs assume valid ringbuf; no error checking

type node struct {
	data int64
	prev *node
	next *node
}

func (n *node) print(num int) {
	if num == 0 {
		fmt.Println()
		return
	}
	fmt.Print(n.data, " ")
	n.next.print(num - 1)
}

func (n *node) getNodeAfter(num int64) *node {
	if num == 0 {
		return n
	}
	return n.next.getNodeAfter(num - 1)
}

func (n *node) insertAfter(node *node) {
	next := n.next

	node.prev = n
	node.next = next
	next.prev = node
	n.next = node

}

func (n *node) unlink() {
	n.prev.next = n.next
	n.next.prev = n.prev
}

func (n *node) moveRight(num int64) {
	if num == 0 {
		return
	}
	next := n.next
	n.unlink()

	next.insertAfter(n)
	n.moveRight(num - 1)
}

func (n *node) moveLeft(num int64) {
	if num == 0 {
		return
	}

	prev := n.prev
	n.unlink()

	prev.prev.insertAfter(n)
	n.moveLeft(num - 1)

}

type doublyLinkedList struct {
	len  int64
	tail *node
	head *node
}

func initDoublyList() *doublyLinkedList {
	return &doublyLinkedList{}
}

func (d *doublyLinkedList) Append(data int64) *node {
	newNode := &node{
		data: data,
	}
	if d.head == nil {
		d.head = newNode
		d.tail = newNode
	} else {
		newNode.prev = d.tail
		d.tail.next = newNode
		d.tail = newNode
	}
	d.len++
	return newNode
}
func (d *doublyLinkedList) TraverseForward() error {
	if d.head == nil {
		return fmt.Errorf("TraverseError: List is empty")
	}
	temp := d.head
	for i := int64(0); i < d.Size(); i++ {
		fmt.Printf("value = %v, this = %p, prev = %p, next = %p\n", temp.data, temp, temp.prev, temp.next)
		temp = temp.next
	}
	fmt.Println()
	return nil
}

func (d *doublyLinkedList) toRingBuf() {
	if d.tail == nil || d.head == nil {
		log.Fatal("not head or tail")
	}
	if d.tail.next != nil || d.head.prev != nil {
		log.Fatal("invalid state")
	}
	d.tail.next = d.head
	d.head.prev = d.tail
}

func (d *doublyLinkedList) Size() int64 {
	return d.len
}
