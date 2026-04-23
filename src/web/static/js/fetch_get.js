
//Функция для отправки GET-запроса:
export async function fetch_get(url) {
  try{
    const response = await fetch(url);
    if(!response.ok){
      throw new Error("Error sending the request");
    }

    const data = await response.json();
    return data;

  }catch(err) {
    throw err;
  }
}
