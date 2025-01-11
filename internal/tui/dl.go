package tui

import (
	"fmt"
	"io"
	"strings"
	"time"

	"github.com/charmbracelet/bubbles/progress"
	tea "github.com/charmbracelet/bubbletea"
	getter "github.com/hashicorp/go-getter"
	"github.com/quickemu-project/quickget_configs/pkg/quickgetdata"
)

type downloader struct {
	path      string
	sources   []quickgetdata.WebSource
	downloads []download
}

type download struct {
	active     bool
	percentage *float64
	bar        progress.Model
	err        chan error
}

func newDownloader(path string, config quickgetdata.Config) downloader {
	sources := extractSources(config)
	if len(sources) == 0 {
		panic("No URLs found")
	}
	return downloader{path: path, sources: sources}
}

func (d download) TrackProgress(src string, currentSize, totalSize int64, stream io.ReadCloser) io.ReadCloser {
	reader := &progressReader{
		reader:     stream,
		percentage: d.percentage,
		total:      totalSize,
		current:    currentSize,
	}

	return &readCloser{
		Reader: reader,
		close: func() error {
			return stream.Close()
		},
	}
}

type progressReader struct {
	reader     io.Reader
	percentage *float64
	total      int64
	current    int64
}

func (pr *progressReader) Read(p []byte) (int, error) {
	n, err := pr.reader.Read(p)
	if n > 0 {
		pr.current += int64(n)
		*pr.percentage = float64(pr.current) / float64(pr.total)
	}
	return n, err
}

type readCloser struct {
	io.Reader
	close func() error
}

func (r *readCloser) Close() error {
	return r.close()
}

type dlStart []download

func (d downloader) Init() tea.Cmd {
	_ = getter.HttpGetter{
		XTerraformGetDisabled: true,
		ReadTimeout:           20 * time.Second,
	}
	return func() tea.Msg {
		bars := make([]download, len(d.sources))
		for i, s := range d.sources {
			go func() {
				bars[i].percentage = new(float64)
				bars[i].active = true
				path := d.path
				url := s.URL
				if s.FileName != "" {
					path = path + "/" + s.FileName
				}
				if s.Checksum != "" {
					url += "?checksum=" + s.Checksum
				}
				client := getter.Client{
					Src:              url,
					Dst:              path,
					Pwd:              ".",
					ProgressListener: bars[i],
					Mode:             getter.ClientModeFile,
				}
				err := client.Get()
				bars[i].err <- err
			}()
		}
		return dlStart(bars)
	}
}

type dlMsg struct {
	msg tea.Cmd
	dl  *download
}

func (d downloader) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	var batch tea.BatchMsg
	switch msg := msg.(type) {
	case dlStart:
		d.downloads = []download(msg)
		cmds := make([]tea.Cmd, len(d.downloads))
		for i := range d.downloads {
			var cmd tea.Cmd
			var bar tea.Model
			bar, cmd = d.downloads[i].bar.Update(msg)
			d.downloads[i].bar = bar.(progress.Model)
			cmds[i] = cmd
		}
		return d, tea.Batch(cmds...)
	case tea.WindowSizeMsg:
		for i := range d.downloads {
			d.downloads[i].bar.Width = msg.Width
		}
	case tea.KeyMsg:
		if msg.Type.String() == "ctrl+c" {
			return d, tea.Quit
		}
	case dlMsg:
		fmt.Println(msg.dl)
		bar, cmd := msg.dl.bar.Update(msg.msg)
		msg.dl.bar = bar.(progress.Model)
		return d, func() tea.Msg { return dlMsg{msg: cmd, dl: msg.dl} }
	}
	for i := range d.downloads {
		if !d.downloads[i].active {
			continue
		}
		batch = append(batch, func() tea.Msg {
			return dlMsg{msg: d.downloads[i].bar.SetPercent(*d.downloads[i].percentage), dl: &d.downloads[i]}
		})
	}
	return d, tea.Batch(batch...)
}

func (d downloader) View() string {
	var s strings.Builder
	for _, bar := range d.downloads {
		s.WriteRune('\n')
		s.WriteString(bar.bar.View())
	}

	return mainStyle.Render(s.String())
}
