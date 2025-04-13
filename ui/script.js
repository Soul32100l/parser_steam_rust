import { invoke } from "@tauri-apps/api/tauri";

async function startParsing() {
  const discount = parseInt(document.getElementById("discount").value);
  const pages = parseInt(document.getElementById("pages").value);

  if (isNaN(discount) || isNaN(pages)) {
    alert("Введите корректные данные!");
    return;
  }

  const resultElement = document.getElementById("result");
  resultElement.textContent = "⏳ Парсинг...";

  try {
    const result = await invoke("parse_steam", {
      discountFilter: discount,
      maxPages: pages,
    });
    resultElement.textContent = result.join("\n");
  } catch (err) {
    resultElement.textContent = "❌ Ошибка: " + err;
  }
}
window.startParsing = startParsing;
