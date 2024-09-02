package main

import (
	"fmt"
	"strings"

	"github.com/gocolly/colly/v2"
)

type Wallpaper struct {
	URL        string
	Preview    string
	Resolution string
}

const BASE_WALLPAPER_SOURCE = "https://wallpaperflare.com/search?wallpaper="

func formatQuery(query string) string {
	return strings.ReplaceAll(query, " ", "+")
}

func Scrape(query string) ([]*Wallpaper, error) {

	query = formatQuery(query)
	var catalogue []*Wallpaper

	collector := colly.NewCollector()
	collector.Limit(&colly.LimitRule{
		DomainGlob: "*",
	})

	collector.OnHTML("ul[itemtype='http://schema.org/ImageGallery'][class='gallery'][id='gallery']", func(h *colly.HTMLElement) {
		h.ForEach("li[itemprop='associatedMedia'][itemscope][itemtype='http://schema.org/ImageObject']", func(_ int, el *colly.HTMLElement) {

			imagePreview := el.ChildAttr("figure a img[itemprop='contentUrl']", "data-src")
			imageURL := el.ChildAttr("figure a[itemprop='url']", "href")

			imageResolution := strings.TrimSpace(el.ChildText("span.res"))

			if imageURL == "" {
				imageURL = el.ChildAttr("figure a img[itemprop='contentUrl']", "src")
			}

			wallpaper := &Wallpaper{
				URL:        imageURL,
				Preview:    imagePreview,
				Resolution: imageResolution,
			}
			catalogue = append(catalogue, wallpaper)
		})
	})

	err := collector.Visit(query)
	if err != nil {
		return nil, err
	}

	if len(catalogue) == 0 {
		return nil, fmt.Errorf("No wallpapers found for query: %s", query)
	}
	collector.Wait()

	return catalogue, nil
}

func main() {

	catalogue, err := Scrape("https://www.wallpaperflare.com/search?wallpaper=inception")

	if err != nil {
		fmt.Println("Error encountered: ", err)
	}

}
