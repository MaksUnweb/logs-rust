import { exit_event, open_event } from "./header.js";

//Здесь запускается код, который необходим именно для всех шаблонов, 
//к примеру для обработки header:

document.addEventListener("DOMContentLoaded", () => {
  let currentModal = document.querySelector(".header-modal");
  open_event(currentModal);
  exit_event(currentModal);
});
