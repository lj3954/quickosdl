package tui

import (
	"slices"

	"github.com/charmbracelet/bubbles/list"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/quickemu-project/quickget_configs/pkg/quickgetdata"
)

type optionSelection struct {
	os   quickgetdata.OSData
	list list.Model
	t    optionType
}

type optionType int

const (
	optionTypeRelease = optionType(iota)
	optionTypeEdition
)

type configItem struct {
	quickgetdata.Config
	t optionType
}

func (c configItem) Title() string {
	switch c.t {
	case optionTypeRelease:
		return c.Release
	case optionTypeEdition:
		return c.Edition
	}
	panic("unreachable")
}
func (c configItem) Description() string {
	return ""
}
func (c configItem) FilterValue() string {
	return c.Title()
}

func newOptionSelection(t optionType, os quickgetdata.OSData, configs []quickgetdata.Config, w, h int) optionSelection {
	if t == optionTypeRelease {
		configs = os.Releases
	}
	var items []list.Item
	set := make(map[string]struct{})
	for _, c := range configs {
		item := configItem{c, t}
		t := item.Title()
		if _, e := set[t]; e {
			continue
		}
		items = append(items, item)
		set[t] = struct{}{}
	}

	d := list.NewDefaultDelegate()
	d.ShowDescription = false
	l := list.New(items, d, w, h)

	switch t {
	case optionTypeRelease:
		l.Title = "Select a Release"
	case optionTypeEdition:
		l.Title = "Select an Edition"
	}

	return optionSelection{os: os, list: l, t: t}
}

func (o optionSelection) Init() tea.Cmd {
	return nil
}

func filterConfigs(configs []quickgetdata.Config, release string) []quickgetdata.Config {
	return slices.DeleteFunc(slices.Clone(configs), func(c quickgetdata.Config) bool {
		return c.Release != release
	})
}

func (o optionSelection) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
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
				switch o.t {
				case optionTypeRelease:
					rel := o.list.SelectedItem().(configItem).Release
					configs := filterConfigs(o.os.Releases, rel)
					if len(configs) > 2 || len(configs) == 1 && configs[0].Edition != "" {
						optSel := newOptionSelection(optionTypeEdition, o.os, configs, o.list.Width(), o.list.Height())
						return optSel, optSel.Init()
					}
					panic("Unimplemented: No editions to select")
				case optionTypeEdition:
					panic("Unimplemented: selected edition")
				}
				arch := quickgetdata.Arch(o.list.SelectedItem().(listArch))
				osSel := newOSSelection(arch, o.list.Width(), o.list.Height())
				return osSel, osSel.Init()
			}
		case "left", "h":
			if !o.list.SettingFilter() {
				switch o.t {
				case optionTypeRelease:
					arch := o.list.Items()[0].(configItem).Arch
					osSel := newOSSelection(arch, o.list.Width(), o.list.Height())
					return osSel, osSel.Init()
				case optionTypeEdition:
					optSel := newOptionSelection(optionTypeRelease, o.os, o.os.Releases, o.list.Width(), o.list.Height())
					return optSel, optSel.Init()
				}
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

func (o optionSelection) View() string {
	return mainStyle.Render(o.list.View())
}
