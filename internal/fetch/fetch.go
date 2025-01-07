package fetch

import (
	"compress/gzip"
	"encoding/json"
	"net/http"
	"slices"

	"github.com/quickemu-project/quickget_configs/pkg/quickgetdata"
)

const gzQgData = "https://github.com/lj3954/quickget_cigo/releases/download/daily/quickget_data.json.gz"

func FetchOsList() ([]quickgetdata.OSData, error) {
	data, err := http.Get(gzQgData)
	if err != nil {
		return nil, err
	}
	defer data.Body.Close()
	gzReader, err := gzip.NewReader(data.Body)
	if err != nil {
		return nil, err
	}
	defer gzReader.Close()

	dec := json.NewDecoder(gzReader)
	var list []quickgetdata.OSData
	if err := dec.Decode(&list); err != nil {
		return nil, err
	}
	list = removeUnwantedEntries(list)

	return list, nil
}

func removeUnwantedEntries(list []quickgetdata.OSData) []quickgetdata.OSData {
	for i := range list {
		list[i].Releases = slices.DeleteFunc(list[i].Releases, func(r quickgetdata.Config) bool {
			return anyNonWebSource(r.ISO) ||
				anyNonWebSource(r.IMG) ||
				anyNonWebSource(r.FixedISO) ||
				anyNonWebSource(r.Floppy) ||
				len(r.DiskImages) != 0
		})
	}
	return slices.DeleteFunc(list, func(o quickgetdata.OSData) bool {
		return len(o.Releases) == 0
	})
}

func anyNonWebSource(sources []quickgetdata.Source) bool {
	for _, s := range sources {
		if s.Custom || s.FileName != "" || s.Docker != nil {
			return true
		}
	}
	return false
}
