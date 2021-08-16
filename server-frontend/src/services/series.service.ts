type Series = {
  getSeries(): Promise<Array<Series>>;
  show: {
    id: number;
    name: string;
    url: string;
    image: { medium: string; original: string };
  };
};

function getSeries(): Promise<Array<Series>> {
  return fetch("http://api.tvmaze.com/search/shows?q=bad").then((res) =>
    res.json()
  );
}

export default {
  getSeries,
};
