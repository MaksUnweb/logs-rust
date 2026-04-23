import { hintEvent } from "./module_hint.js";
import { handler_backend_notify } from "../module_close_notify.js";

document.addEventListener("DOMContentLoaded", () => {
  hintEvent();
  handler_backend_notify();
});
