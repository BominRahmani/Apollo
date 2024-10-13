import './style.css';
import './app.css';

import logo from './assets/images/logo-universal.png';
import {Scrape, DownloadImages} from '../wailsjs/go/main/App';

let searchInput = document.getElementById("search-input");
searchInput.focus();


let swiper;
let searchInputEl;


async function search_wallpapers() {

  const statusMessageEl = document.getElementById('status-message')
  const searchInputEl = document.getElementById('search-input');
  

  try {
    const rawResults = await Scrape(searchInputEl.value);
    await loadImages(rawResults);

  } catch (error) {
    console.error("Error parsing results:", error);
  }
}

async function loadImages(results) {
  const swiperWrapper = document.querySelector('.swiper-wrapper');
  swiperWrapper.innerHTML = ''; 

  for (const wallpaper of results) {
    console.log("This is wallpaper:" + wallpaper.URL);
    const slide = document.createElement('div');
    slide.className = 'swiper-slide';

    const img = document.createElement('img');
    const wallpaper_src = wallpaper.Preview;
    img.src = wallpaper_src;
    img.alt = `placeholder`;

    const resolution = document.createElement('p');
    resolution.textContent = wallpaper.Resolution;

    slide.appendChild(img);
    slide.appendChild(resolution);

    
     // Add onClick event
    slide.addEventListener('click', () => {
      console.log("GOT CLICKED");
      handleSlideClick(wallpaper.URL);
    });


    swiperWrapper.appendChild(slide); 
  }
   // Reinitialize or update Swiper
  if (swiper) {
    swiper.update();
  } else {
    initSwiper();
  }
}


function initSwiper() {
  swiper = new Swiper(".swiper", {
    grabCursor: true,
    initialSlide: 0,
    centeredSlides: true,
    slidesPerView: 3,
    spaceBetween: 30,
    speed: 1000,
    freeMode: true,
    mousewheel: {
      thresholdDelta: 30,
    },
    on: {
      click(event) {
        swiper.slideTo(this.clickedIndex);
      },
    },
  });
}


async function handleSlideClick(image_url) {
  await DownloadImages(image_url);
}


document.addEventListener('DOMContentLoaded', function() {
    const searchContainer = document.getElementById('search-container');
    const searchInput = document.getElementById('search-input');

    searchInput.addEventListener('keyup', function(event) {
      if (event.key === 'Enter') {
        const searchTerm = searchInput.value.trim();
        if (searchTerm !== '') {
          search_wallpapers();
          hideSearch();
          searchInput.value = ''; 
        }
      }
    });

    document.addEventListener('keydown', function(event) {
      if (event.key === 'Escape') {
        event.preventDefault(); // Prevent default behavior (including the beep)
        toggleSearch();
      }
    });

    function hideSearch() {
      searchContainer.classList.add('hidden');
    }

    function showSearch() {
      searchContainer.classList.remove('hidden');
      searchInput.focus(); // Optional: focus on the input when shown
    }

    function toggleSearch() {
      if (searchContainer.classList.contains('hidden')) {
        showSearch();
      } else {
        hideSearch();
      }
    }

    initSwiper();
});
