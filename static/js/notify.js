export function notify(data, is_error) {
  
  let popup = document.querySelector(".notify-modal");
  if (popup === null) { 
    return;
  }
  
  let p = document.createElement("p");
  p.textContent = data;

  if (!is_error)  {
    popup.classList.add("success-modal");
    popup.appendChild(p);
    popup.style.display = "flex";
  }else{
    popup.classList.add("error-modal");
    popup.appendChild(p);
    popup.style.display = "flex";
  }
  
  let duration = 200;
  setTimeout(() => {
    // popup.style.backgroundColor = "#173f8f";
    open_notify(popup, duration);
  }, 200)

  setTimeout(() => {
    close_notify(popup, duration);
  }, 2000)
}


function open_notify(popup, duration) {

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

function close_notify(popup, duration) {

  let start_opacity = '1';
  let start = performance.now();

  function animate(time) {
    let timeFraction = (time - start) / duration;
    if (timeFraction > 1) {
      timeFraction = 1
      popup.innerHTML = "";
      popup.classList.remove("success-modal", "error-modal");
      popup.style.display = "none";
    }

    let current_opacity = start_opacity * (1 - timeFraction);
    popup.style.opacity = `${current_opacity}`;

    if(timeFraction < 1) {
      requestAnimationFrame(animate);
    }
  }
  requestAnimationFrame(animate);
}
