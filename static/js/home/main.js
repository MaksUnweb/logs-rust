import { test_db } from "./benchmark_db.js";
import { select_result } from "./benchmark_db.js";


document.addEventListener("DOMContentLoaded", () => {
  test_db();
  select_result();
});
