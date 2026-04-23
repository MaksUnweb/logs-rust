const DURATION = 200;

export function hintEvent() {
  let tooltip = document.querySelectorAll(".hint-background");
  let hint_block = document.querySelector(".hint-block");
  tooltip.forEach(el => {
    el.addEventListener("click", (e) => {
      
      if (e.target.tagName === "svg" || e.target.tagName === "use"||e.target.classList.contains("hint-background")) {
        let span = "";
        if (e.target.tagName === "use") {
          let svg_el = e.target.parentElement;
          let hint_el = svg_el.parentElement;
          let card = hint_el.parentElement;
          span = card.querySelector("span");
        } else if (e.target.tagName === "svg") {
          let hint_el = e.target.parentElement;
          let card = hint_el.parentElement;
          span = card.querySelector("span");
        } else if (e.target.classList.contains("hint-background")) {
          let card = e.target.parentElement;
          span = card.querySelector("span");
        }
          let text = span.textContent;
          openBlock(text, hint_block, DURATION);
      }
    });
  });


  //Добавляем обработчик на main для клика "закрытия":
  document.querySelector("main").addEventListener("click", (e) => {
    if (!e.target.closest(".hint-background") && !e.target.closest(".hint-block")) {
        closeBlock(hint_block, DURATION);
    }
  })
}

function openBlock(text, hint_block, duration) {
  hint_block.innerHTML = "";
  let main = document.querySelector("main");
  let span = document.createElement("span");
  span.textContent = text;
  hint_block.appendChild(span);
  hint_block.style.display = "flex";
  
  let start_opacity = '0';
  let start = performance.now(); 

  function animate(time) {
    let timeFraction = (time - start) / duration;
    if (timeFraction > 1) timeFraction = 1;

    let current_opacity = timeFraction;
    hint_block.style.opacity = `${current_opacity}`;

    if (timeFraction < 1) {
      requestAnimationFrame(animate);
    }
  }

  requestAnimationFrame(animate);
  
}

function closeBlock(hint_block, duration) {
  
  let start_opacity = '1';
  let start = performance.now();

  function animate(time) {
    let timeFraction = (time - start) / duration;
    if (timeFraction > 1) {
      timeFraction = 1

      hint_block.style.display = "none";
      hint_block.innerHTML = "";
    }

    let current_opacity = start_opacity * (1 - timeFraction);
    hint_block.style.opacity = `${current_opacity}`;

    if(timeFraction < 1) {
      requestAnimationFrame(animate);
    }
  }
  requestAnimationFrame(animate);
  
}
