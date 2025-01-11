package tui

import (
	"github.com/charmbracelet/bubbles/filepicker"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/quickemu-project/quickget_configs/pkg/quickgetdata"
)

type filePicker struct {
	config quickgetdata.Config
	model  filepicker.Model
}

func newFilePicker(config quickgetdata.Config, y int) filePicker {
	fp := filepicker.New()
	fp.DirAllowed = true
	fp.FileAllowed = false
	fp.Height = y - 5
	return filePicker{config: config, model: fp}
}

func (f filePicker) Init() tea.Cmd {
	return f.model.Init()
}

func (f filePicker) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "ctrl+c", "q":
			return f, tea.Quit
		case "enter":
			dir := f.model.CurrentDirectory
			dl := newDownloader(dir, f.config)
			return dl, dl.Init()
		}
	}
	var cmd tea.Cmd
	f.model, cmd = f.model.Update(msg)
	return f, cmd
}

func (f filePicker) View() string {
	return mainStyle.Render("Select a directory\n" + f.model.View())
}
