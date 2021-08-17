type Players = {
  fetchPlayers(): Promise<Players>;
  name: string;
};

async function fetchPlayers(): Promise<Players> {
  const response = await window.fetch("http://planetoid:8080/players", {
    method: "GET",
    headers: {
      "content-type": "application/json;charset=UTF-8",
    },
  });

  const data = await response.json();
  if (response.ok) {
    return Object.assign(data);
  } else {
    return Promise.reject(new Error(`Error retriving players.`));
  }
}

export default {
  fetchPlayers,
};
