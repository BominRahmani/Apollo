const { invoke, convertFileSrc } = window.__TAURI__.tauri;


let swiper;
let searchInputEl;
const wallpaperGrid = document.getElementById('wallpaper-grid');


async function search_wallpapers() {

  const statusMessageEl = document.getElementById('status-message')
  const searchInputEl = document.getElementById('search-input');
  

  try {
    const rawResults = await invoke("scrape_wallpapers", { query: searchInputEl.value });
    const results = JSON.parse(rawResults);

    console.log(rawResults);
    await loadFolders(results);

    console.log("Parsed results:", results);
  } catch (error) {
    console.error("Error parsing results:", error);
  }
}


async function loadFolders(results) {
  const cardWrapper = document.querySelector('.cards-list');
  cardWrapper.innerHTML = '';

  for (const file of results) {
    console.log(file);
    const card = document.createElement('div');
    card.className = 'card';

    // card image
    const card_img = document.createElement('div');
    card_img.className = 'card_image';
    const img = document.createElement('img');
    const file_preview = file.preview;
    img.src = file_preview;
    img.alt = `placeholder alt img`;
   
    card_img.appendChild(img);

    // card title
    const card_title = document.createElement('div');
    card_title.className = 'card_title title-white';
    const card_text = document.createElement('p');
    const file_text_preview = file.title + " " + file.num_imgs;
    card_text.innerText = file_text_preview;

    card_title.appendChild(card_text);

    card.appendChild(card_img);
    card.appendChild(card_title);
    cardWrapper.appendChild(card);
  }
}



async function loadImages(results) {
  const swiperWrapper = document.querySelector('.swiper-wrapper');
  swiperWrapper.innerHTML = ''; 

  for (const wallpaper of results) {
    const slide = document.createElement('div');
    slide.className = 'swiper-slide';

    const img = document.createElement('img');
    const wallpaper_src = convertFileSrc(wallpaper.local_preview);
    img.src = wallpaper_src;
    img.alt = `Wallpaper ${wallpaper.resolution}`;

    const resolution = document.createElement('p');
    resolution.textContent = wallpaper.resolution;

    slide.appendChild(img);
    slide.appendChild(resolution);
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

document.addEventListener('DOMContentLoaded', function() {
    const searchContainer = document.getElementById('search-container');
    const searchInput = document.getElementById('search-input');
    const cardsList = document.getElementById('cards-list');

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
      cardsList.classList.remove('hidden');
      searchContainer.classList.add('hidden');

    }

    function showSearch() {
      cardsList.classList.add('hidden');
      searchContainer.classList.remove('hidden');
      searchInput.focus(); // Optional: focus on the input when shown
    }

    initSwiper();
});
