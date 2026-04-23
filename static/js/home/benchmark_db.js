import { fetch_get } from "../fetch_get.js";
import { notify } from "../notify.js";

export async function test_db() {
  let test_btn = document.querySelector("#benchmark-test");
  test_btn.addEventListener("click", async (e) => {
    e.preventDefault();

    let url = "/api/test_db";
    try{
      let result = await fetch_get(url);
      notify(result.message, false);
      select_result();
    }catch(err) {
      notify(err.message, true);
    }
  });
}
// Функция для вывода последнего теста
export async function select_result() {
  let url = "/api/select_test_db";
  try{
    let result = await fetch_get(url);
    let info_class = document.querySelector(".info");
    info_class.innerHTML = "";
    let span_date = document.createElement("span");
    let span_speed = document.createElement("span");
    if (result.data !== null){
      span_date.innerHTML = `<strong>Past test: </strong>${result.data.date}`;
      span_speed.innerHTML = `<strong>Speed: </strong>${result.data.time}ms`;
    }else{
      span_date.innerHTML = `<strong>Past test: -</strong>`;
      span_speed.innerHTML = `<strong>Speed: -</strong>`;
    }
      info_class.appendChild(span_date);
      info_class.appendChild(span_speed);

  }catch(err){
      notify(err.message, true);
  }
}
