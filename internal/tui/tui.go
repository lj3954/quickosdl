package tui

import (
	"log"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
)

func Run() {
	go func() {
		if _, err := osListFunc(); err != nil {
			panic("Error while initializing OS List: " + err.Error())
		}
	}()

	model := newArchSelection()

	p := tea.NewProgram(model)
	if _, err := p.Run(); err != nil {
		log.Fatalln(err)
	}
}

var mainStyle = lipgloss.NewStyle()
