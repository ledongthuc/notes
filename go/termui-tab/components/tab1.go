package components

import (
	ui "github.com/gizak/termui/v3"
	"github.com/gizak/termui/v3/widgets"
)

func GetTab1() *widgets.Paragraph {
	p2 := widgets.NewParagraph()
	p2.Text = "Press q to quit\nPress h or l to switch tabs\n"
	p2.Title = "Keys"
	p2.SetRect(5, 5, 40, 15)
	p2.BorderStyle.Fg = ui.ColorYellow
	return p2
}
