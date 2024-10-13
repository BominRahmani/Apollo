package main

import (
	"context"
	"fmt"
	"strings"

	"github.com/gocolly/colly/v2"
)

// App struct
type App struct {
	ctx context.Context
}

// NewApp creates a new App application struct
func NewApp() *App {
	return &App{}
}

// startup is called when the app starts. The context is saved
// so we can call the runtime methods
func (a *App) startup(ctx context.Context) {
	a.ctx = ctx
}

// // Greet returns a greeting for the given name
// func (a *App) Greet(name string) string {
//   catalogue, err := Scrape("https://www.wallpaperflare.com/search?wallpaper=inception")
//   fmt.Println(catalogue)
//
// 	if err != nil {
// 		fmt.Println("Error encountered: ", err)
// 	}
// 	return fmt.Sprintf("Hello %s, It's show time!", catalogue)
// }

type Wallpaper struct {
	URL        string
	Preview    string
	Resolution string
}

const BASE_WALLPAPER_SOURCE = "http://wallpaperflare.com/search?wallpaper="

func formatQuery(query string) string {
	return strings.ReplaceAll(query, " ", "+")
}

func (a *App) Scrape(query string) ([]*Wallpaper, error) {

	query = BASE_WALLPAPER_SOURCE + formatQuery(query)
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


