package components

import (
	"github.com/gizak/termui/v3/widgets"
)

func GetTabPane() *widgets.TabPane {
	tabpane := widgets.NewTabPane("Play", "Settings")
	tabpane.SetRect(0, 0, 19, 1)
	tabpane.PaddingLeft = 1
	tabpane.Border = false
	return tabpane
}
