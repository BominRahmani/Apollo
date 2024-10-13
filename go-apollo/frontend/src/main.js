import './style.css';
import './app.css';

import logo from './assets/images/logo-universal.png';
import {Scrape} from '../wailsjs/go/main/App';

let searchInput = document.getElementById("search-input");
searchInput.focus();

//// Setup the greet function
//window.greet = function () {
//    // Get name
//    let query = searchInput.value;
//
//    // Check if the input is empty
//    if (name === "") return;
//
//    // Call App.Greet(name)
//    try {
//        Scrape(query)
//            .then((result) => {
//                // Update result with data back from App.Greet()
//              console.log(result)
//            })
//            .catch((err) => {
//                console.error(err);
//            });
//    } catch (err) {
//        console.error(err);
//    }
//};
//
//

let swiper;
let searchInputEl;


async function search_wallpapers() {

  const statusMessageEl = document.getElementById('status-message')
  const searchInputEl = document.getElementById('search-input');
  

  try {
    const rawResults = await Scrape(searchInputEl.value);
    //console.log(rawResults);

    //const results = JSON.parse(rawResults);

    await loadImages(rawResults);

    //console.log("Parsed results:", results);
  } catch (error) {
    console.error("Error parsing results:", error);
  }
}

async function loadImages(results) {
  const swiperWrapper = document.querySelector('.swiper-wrapper');
  swiperWrapper.innerHTML = ''; 

  for (const wallpaper of results) {
    console.log("This is wallpaper:" + wallpaper.Preview);
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
      handleSlideClick(wallpaper.url);
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
    pagination: {
      el: ".swiper-pagination",
    },
    on: {
      click(event) {
        swiper.slideTo(this.clickedIndex);
      },
    },
  });
}


async function handleSlideClick(fileUrl) {
 await invoke("choose_wallpaper", { url : fileUrl});
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
        showSearch();
      }
    });

    function hideSearch() {
      searchContainer.classList.add('hidden');

    }

    function showSearch() {
      searchContainer.classList.remove('hidden');
      searchInput.focus(); // Optional: focus on the input when shown
    }

    initSwiper();
});
