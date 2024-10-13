package main

import (
	"context"
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"time"

	"github.com/gocolly/colly/v2"
  "github.com/reujab/wallpaper"

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
				URL:        imageURL + "/download",
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

func (a *App) DownloadImages(url string) (string, error) {
	resp, err := http.Get(url)
	if err != nil {
		return "", fmt.Errorf("error making HTTP request: %v", err)
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return "", fmt.Errorf("error reading response body: %v", err)
	}

	re := regexp.MustCompile(`show_img" src="([^"]*)"`)
	matches := re.FindSubmatch(body)

	if len(matches) < 2 {
		return "", fmt.Errorf("image link not found in the HTML")
	}

  imageURL := string(matches[1])

  // Create a temporary directory in the user's home folder
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return "", fmt.Errorf("error getting user home directory: %v", err)
	}

	tempDir := filepath.Join(homeDir, "apollo_images")
	err = os.MkdirAll(tempDir, 0755)
	if err != nil {
		return "", fmt.Errorf("error creating temporary directory: %v", err)
	}

	// Generate a unique filename for the image
	filename := filepath.Join(tempDir, fmt.Sprintf("image_%d.jpg", time.Now().UnixNano()))

	// Download the image
	imageResp, err := http.Get(imageURL)
	if err != nil {
		return "", fmt.Errorf("error downloading image: %v", err)
	}
	defer imageResp.Body.Close()

	// Create the file
	file, err := os.Create(filename)
	if err != nil {
		return "", fmt.Errorf("error creating file: %v", err)
	}
	defer file.Close()

	// Write the image data to the file
	_, err = io.Copy(file, imageResp.Body)
	if err != nil {
		return "", fmt.Errorf("error saving image: %v", err)
	}


  wallpaper.SetMode(wallpaper.Fit)
  wallpaper.SetFromFile(filename)

	return string(matches[1]), nil
}




