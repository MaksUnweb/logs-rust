
// Добавляем обработчик для кнопки открытия:
export function open_event(currentModal) {
  let btn = document.querySelector(".header-open");
  btn.addEventListener("click", (e) => {
    e.preventDefault();
    open_header_modal(currentModal);
  });
}


//Добавляем обработчик для кнопки закрытия:
export function exit_event(currentModal) {
  let btn = currentModal.querySelector("svg");
  btn.addEventListener("click", (e) => {
    e.preventDefault();

    close_header_modal(currentModal);
  }) 
}

//Функция для открытия модального окна:
function open_header_modal(modal) {
    modal.style.display = "flex";
  let window_width = document.documentElement.clientWidth;
  let start_position = ( window_width / 2);
  let duration = 700; 
  let start = performance.now();

  function easeOutQuad(timeFraction) {
    return 1 - Math.pow(1 - timeFraction, 2);
  }


  function animate(time) {
    let timeFraction = (time - start) / duration;
    if (timeFraction > 1) timeFraction = 1;

    let progress = easeOutQuad(timeFraction);
    let current_position = start_position * (1 - progress);
    
    modal.style.transform = `translateX(${current_position}px)`;

    if (timeFraction < 1) {
      requestAnimationFrame(animate);
    }
  }

  requestAnimationFrame(animate);
}


//Функция для закрытия модального окна:
function close_header_modal(modal) {

  let window_width = document.documentElement.clientWidth;
  let start_position = window_width / 2;
  let duration = 700;
  let start = performance.now();

  function easeOutQuad(timeFraction) {
    return 1 - Math.pow(1 - timeFraction, 2);
  }

  function animate(time) {
    let timeFraction = (time - start) / duration;
    if(timeFraction > 1) timeFraction = 1;

    let progress = easeOutQuad(timeFraction);
    let current_position = start_position * progress;

    modal.style.transform = `translateX(${current_position}px)`;

    if(timeFraction < 1) {
      requestAnimationFrame(animate);
    }else{
      modal.style.display = "none";
    }
  }
  requestAnimationFrame(animate);
}
