package tui

import (
	"slices"
	"sync"

	"github.com/charmbracelet/bubbles/list"
	"github.com/charmbracelet/bubbles/spinner"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/lj3954/quickosdl/internal/fetch"
	"github.com/quickemu-project/quickget_configs/pkg/quickgetdata"
)

var osListFunc = sync.OnceValues(fetch.FetchOsList)

func getOsList(arch quickgetdata.Arch) osList {
	list, err := osListFunc()
	if err != nil {
		panic("Error while fetching OS list. This should never happen (os list should be fetched on init)")
	}
	// We clone the list so that the original list is not mutated, which would cause a bug if the arch was modified
	list = slices.Clone(list)
	for i := range list {
		list[i].Releases = slices.DeleteFunc(slices.Clone(list[i].Releases), func(r quickgetdata.Config) bool {
			if arch == quickgetdata.X86_64 {
				return r.Arch != "" && r.Arch != arch
			}
			return r.Arch != arch
		})
	}
	list = slices.DeleteFunc(list, func(os quickgetdata.OSData) bool {
		return len(os.Releases) == 0
	})

	return osList(list)
}

type listOS struct {
	os quickgetdata.OSData
}

func (l listOS) Title() string {
	return l.os.PrettyName
}
func (l listOS) Description() string {
	return l.os.Description
}
func (l listOS) FilterValue() string {
	return l.os.Name
}

type osSelection struct {
	arch quickgetdata.Arch
	list list.Model
}

func newOSSelection(arch quickgetdata.Arch, w, h int) osSelection {
	l := list.New([]list.Item{}, list.NewDefaultDelegate(), w, h)
	l.Title = "Select an Operating System"
	l.SetSpinner(spinner.MiniDot)
	l.StartSpinner()
	return osSelection{arch: arch, list: l}
}

type osList []quickgetdata.OSData

func (o osSelection) Init() tea.Cmd {
	return tea.Batch(
		o.list.StartSpinner(),
		func() tea.Msg {
			return getOsList(o.arch)
		},
	)
}

func (o osSelection) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case osList:
		items := make([]list.Item, len(msg))
		for i, os := range msg {
			items[i] = listOS{os}
		}
		o.list.SetItems(items)
		o.list.StopSpinner()
	case tea.KeyMsg:
		switch msg.String() {
		case "ctrl+c":
			return o, tea.Quit
		case "q":
			if !o.list.SettingFilter() {
				return o, tea.Quit
			}
		case "enter", "right", "l":
			if !o.list.SettingFilter() && o.list.SelectedItem() != nil {
				os := o.list.SelectedItem().(listOS).os
				configs := os.Releases
				optSel := newOptionSelection(optionTypeRelease, os, configs, o.list.Width(), o.list.Height())
				return optSel, optSel.Init()
			}
		case "left", "h":
			if !o.list.SettingFilter() {
				archSel := newArchSelection()
				archSel.list.SetSize(o.list.Width(), o.list.Height())
				return archSel, archSel.Init()
			}
		}
	case tea.WindowSizeMsg:
		h, v := mainStyle.GetFrameSize()
		o.list.SetSize(msg.Width-h, msg.Height-v)
	}

	var cmd tea.Cmd
	o.list, cmd = o.list.Update(msg)
	return o, cmd
}

func (o osSelection) View() string {
	return mainStyle.Render(o.list.View())
}
