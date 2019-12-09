package main

import (
	"fmt"
	"log"

	"github.com/jroimartin/gocui"
)

var content = "Hello world! Type or click anything"
var viewName = "helloView"

func main() {
	g, err := gocui.NewGui(gocui.OutputNormal)
	if err != nil {
		log.Panicln(err)
	}
	defer g.Close()

	g.Cursor = true
	g.Mouse = true
	g.SetManagerFunc(layout)

	if err := g.SetKeybinding("", gocui.KeyCtrlC, gocui.ModNone, quit); err != nil {
		log.Panicln(err)
	}
	if err := g.SetKeybinding(viewName, gocui.MouseLeft(), gocui.ModNone, onClick); err != nil {
		log.Panicln(err)
	}
	if err := g.SetKeybinding(viewName, gocui.MouseLeft(), gocui.ModNone, onType); err != nil {
		log.Panicln(err)
	}

	if err := g.MainLoop(); err != nil && err != gocui.ErrQuit {
		log.Panicln(err)
	}
}

func onClick(g *gocui.Gui, v *gocui.View) error {
	content = "Left mouse click!"
	v, err := g.View(viewName)
	if err != nil {
		return err
	}
	v.Clear()
	fmt.Fprintln(v, content)
	return err
}

func onType(g *gocui.Gui, v *gocui.View) error {
	content = "on Type"
	v, err := g.View(viewName)
	if err != nil {
		return err
	}
	v.Clear()
	fmt.Fprintln(v, content)
	return err
}

func layout(g *gocui.Gui) error {
	maxX, maxY := g.Size()
	width, height := 36, 2
	if v, err := g.SetView(viewName, maxX/2-width/2, maxY/2, maxX/2+width/2, maxY/2+height); err != nil {
		if err != gocui.ErrUnknownView {
			return err
		}
		fmt.Fprintln(v, content)
	}
	return nil
}

func quit(g *gocui.Gui, v *gocui.View) error {
	return gocui.ErrQuit
}
