package main

import (
	"bufio"
	"fmt"
	"log"
	"os"

	ui "github.com/gizak/termui/v3"
	"github.com/ledongthuc/snippet/go/termui-tab/components"
)

func main() {
	fmt.Println("Type anything to start")
	fmt.Print("> ")
	buf := bufio.NewReader(os.Stdin)
	sentence, err := buf.ReadString('\n')
	if err != nil {
		fmt.Println(err)
	} else {
		fmt.Println(string(sentence))
	}

	if err := ui.Init(); err != nil {
		log.Fatalf("failed to initialize termui: %v", err)
	}
	defer ui.Close()

	p2 := components.GetTab1()
	bc := components.GetTab2()
	tabpane := components.GetTabPane()

	renderTab := func() {
		switch tabpane.ActiveTabIndex {
		case 0:
			ui.Render(p2)
		case 1:
			ui.Render(bc)
		}
	}

	ui.Render(tabpane, p2)
	tabpane.FocusLeft()
	ui.Clear()
	ui.Render(tabpane)
	renderTab()

	uiEvents := ui.PollEvents()
	for {
		e := <-uiEvents
		switch e.ID {
		case "q", "<C-c>":
			return
		case "h":
			tabpane.FocusLeft()
			ui.Clear()
			ui.Render(tabpane)
			renderTab()
		case "l":
			tabpane.FocusRight()
			ui.Clear()
			ui.Render(tabpane)
			renderTab()
		}
	}
}
