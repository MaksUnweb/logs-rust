
export function handler_backend_notify() {
  let popup = document.querySelector(".notify-modal");

  if (popup === null) { 
    return;
  }
  
  let duration = 200;
  setTimeout(() => {
    // popup.style.backgroundColor = "#173f8f";
    open_popup(popup, duration);
  }, 200)

  setTimeout(() => {
    close_popup(popup, duration);
  }, 2000)
}

//функция для открытия popup:
function open_popup(popup, duration) {

  let start_opacity = '0';
  let start = performance.now(); 

  function animate(time) {
    let timeFraction = (time - start) / duration;
    if (timeFraction > 1) timeFraction = 1;

    let current_opacity = timeFraction;
    popup.style.opacity = `${current_opacity}`;

    if (timeFraction < 1) {
      requestAnimationFrame(animate);
    }
  }

  requestAnimationFrame(animate);
}

//Функция для закрытия popup-сообщения об результате выполенения:
function close_popup(popup, duration) {

  let start_opacity = '1';
  let start = performance.now();

  function animate(time) {
    let timeFraction = (time - start) / duration;
    if (timeFraction > 1) {
      timeFraction = 1

      const url = new URL(window.location);
      url.searchParams.delete('message');    
      url.searchParams.delete('is_error');   
      history.replaceState(null, '', url);   

      popup.innerHTML = "";
      popup.remove();
    }

    let current_opacity = start_opacity * (1 - timeFraction);
    popup.style.opacity = `${current_opacity}`;

    if(timeFraction < 1) {
      requestAnimationFrame(animate);
    }
  }
  requestAnimationFrame(animate);
}
