package main

import (
	proxyGreeter "app/gen"
)

func init() {
	a := ProxyGreeterImpl{}
	proxyGreeter.SetHandler(a)
}

type ProxyGreeterImpl struct {
}

func (i ProxyGreeterImpl) greet() string {
	return proxyGreeter.imports.greet() + " and Go ʕ•ᴥ•ʔ!"
}

//go:generate wit-bindgen tiny-go ../../wit/world.wit --world greeter-adapter --out-dir=gen
func main() {}