package tui

import (
	"iter"

	"github.com/charmbracelet/bubbles/list"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/quickemu-project/quickget_configs/pkg/quickgetdata"
)

type urlDisplay struct {
	list   list.Model
	os     quickgetdata.OSData
	config quickgetdata.Config
}

func newUrlDisplay(os quickgetdata.OSData, config quickgetdata.Config, w, h int) urlDisplay {
	urls := extractUrls(config)
	d := list.NewDefaultDelegate()
	d.ShowDescription = false
	l := list.New(urls, d, w, h)
	l.Title = osName(os, config) + " URLs"
	return urlDisplay{list: l, os: os, config: config}
}

func extractUrls(config quickgetdata.Config) []list.Item {
	urls := make([]list.Item, 0, len(config.ISO)+len(config.IMG)+len(config.FixedISO)+len(config.Floppy))
	for s := range sliceIter(config.ISO, config.IMG, config.FixedISO, config.Floppy) {
		urls = append(urls, listItem(s.Web.URL))
	}
	return urls
}

func sliceIter[T any](slices ...[]T) iter.Seq[T] {
	return func(yield func(T) bool) {
		for _, slice := range slices {
			for _, item := range slice {
				if !yield(item) {
					return
				}
			}
		}
	}
}

func (u urlDisplay) Init() tea.Cmd {
	return nil
}

func (u urlDisplay) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "ctrl+c":
			return u, tea.Quit
		case "q":
			if !u.list.SettingFilter() {
				return u, tea.Quit
			}
		case "left", "h":
			if !u.list.SettingFilter() {
				dlPrompt := newDlPrompt(u.os, u.config, u.list.Width(), u.list.Height())
				return dlPrompt, dlPrompt.Init()
			}
		}
	case tea.WindowSizeMsg:
		h, v := mainStyle.GetFrameSize()
		u.list.SetSize(msg.Width-h, msg.Height-v)
	}

	var cmd tea.Cmd
	u.list, cmd = u.list.Update(msg)
	return u, cmd
}

func (u urlDisplay) View() string {
	return mainStyle.Render(u.list.View())
}
