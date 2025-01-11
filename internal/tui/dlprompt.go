package tui

import (
	"strings"

	"github.com/charmbracelet/bubbles/list"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/quickemu-project/quickget_configs/pkg/quickgetdata"
)

type listItem string

func (s listItem) Title() string {
	return string(s)
}
func (s listItem) Description() string {
	return ""
}
func (s listItem) FilterValue() string {
	return string(s)
}

const (
	displayURLs listItem = "Display URLs"
	downloadNow listItem = "Download OS now"
)

func newDlPrompt(os quickgetdata.OSData, config quickgetdata.Config, x, y int) dlPrompt {
	d := list.NewDefaultDelegate()
	d.ShowDescription = false
	l := list.New([]list.Item{displayURLs, downloadNow}, d, x, y)
	l.Title = "Download " + osName(os, config)

	return dlPrompt{os: os, config: config, list: l}
}

func osName(os quickgetdata.OSData, config quickgetdata.Config) string {
	var str strings.Builder
	str.WriteString(os.PrettyName)
	str.WriteString(" ")
	str.WriteString(config.Release)
	if config.Edition != "" {
		str.WriteString(" ")
		str.WriteString(config.Edition)
	}
	return str.String()
}

type dlPrompt struct {
	os     quickgetdata.OSData
	config quickgetdata.Config
	list   list.Model
}

func (d dlPrompt) Init() tea.Cmd {
	return nil
}

func (d dlPrompt) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "ctrl+c":
			return d, tea.Quit
		case "q":
			if !d.list.SettingFilter() {
				return d, tea.Quit
			}
		case "enter", "right", "l":
			if !d.list.SettingFilter() && d.list.SelectedItem() != nil {
				switch d.list.SelectedItem().(listItem) {
				case displayURLs:
					urlDisplay := newUrlDisplay(d.os, d.config, d.list.Width(), d.list.Height())
					return urlDisplay, urlDisplay.Init()
				case downloadNow:
					fp := newFilePicker(d.config, d.list.Height())
					return fp, fp.Init()
				}
			}
		case "left", "h":
			if !d.list.SettingFilter() {
				filteredConfigs := filterConfigs(d.os.Releases, d.config.Release)
				optionSel := newOptionSelection(optionTypeEdition, d.os, filteredConfigs, d.list.Width(), d.list.Height())
				return optionSel, optionSel.Init()
			}
		}
	case tea.WindowSizeMsg:
		h, v := mainStyle.GetFrameSize()
		d.list.SetSize(msg.Width-h, msg.Height-v)
	}

	var cmd tea.Cmd
	d.list, cmd = d.list.Update(msg)
	return d, cmd
}

func (d dlPrompt) View() string {
	return mainStyle.Render(d.list.View())
}
