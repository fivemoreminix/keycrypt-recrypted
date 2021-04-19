package main

import "testing"

func TestEncrypt(t *testing.T) {
	expected := "63999, 29493!"
	if got := EncryptString("Hello, world!"); got != expected {
		t.Errorf("Expected %#v, got %#v", expected, got)
	}
}

func BenchmarkEncrypt(b *testing.B) {
	for i := 0; i < b.N; i++ {
		EncryptString("Perry the platypus is a handsome creature. Honestly, he is undeniably adorable, especially in his suit when Dr. Doofenshmirtz is constructing evil machines.")
	}
}
