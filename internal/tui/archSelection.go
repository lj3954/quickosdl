package tui

import (
	"runtime"
	"slices"

	"github.com/charmbracelet/bubbles/list"
	tea "github.com/charmbracelet/bubbletea"
	"github.com/quickemu-project/quickget_configs/pkg/quickgetdata"
)

type listArch quickgetdata.Arch

func (l listArch) Title() string {
	return string(l)
}
func (l listArch) Description() string {
	return ""
}
func (l listArch) FilterValue() string {
	return string(l)
}

func newArchSelection() archSelection {
	archList := []quickgetdata.Arch{quickgetdata.X86_64, quickgetdata.Aarch64, quickgetdata.Riscv64}
	defaultSelection := slices.Index(archList, sysArch())

	items := make([]list.Item, len(archList))
	for i, arch := range archList {
		items[i] = listArch(arch)
	}

	d := list.NewDefaultDelegate()
	d.ShowDescription = false

	l := list.New(items, d, 0, 0)
	l.Select(defaultSelection)
	return archSelection{list: l}
}

func sysArch() quickgetdata.Arch {
	switch runtime.GOARCH {
	case "arm64":
		return quickgetdata.Aarch64
	case "riscv64":
		return quickgetdata.Riscv64
	default:
		return quickgetdata.X86_64
	}
}

type archSelection struct {
	list list.Model
}

func (a archSelection) Init() tea.Cmd {
	return nil
}

func (a archSelection) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.String() {
		case "ctrl+c":
			return a, tea.Quit
		case "q":
			if !a.list.SettingFilter() {
				return a, tea.Quit
			}
		case "enter", "right", "l":
			if !a.list.SettingFilter() {
				arch := quickgetdata.Arch(a.list.SelectedItem().(listArch))
				osSel := newOSSelection(arch, a.list.Width(), a.list.Height())
				return osSel, osSel.Init()
			}
		}
	case tea.WindowSizeMsg:
		h, v := mainStyle.GetFrameSize()
		a.list.SetSize(msg.Width-h, msg.Height-v)
	}

	var cmd tea.Cmd
	a.list, cmd = a.list.Update(msg)
	return a, cmd
}

func (a archSelection) View() string {
	return mainStyle.Render(a.list.View())
}
