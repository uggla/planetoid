<template>
  <div class="hello">
    <h1>{{ msg }}</h1>
    <p>
      For a guide and recipes on how to configure / customize this project,<br />
      check out the
      <a href="https://cli.vuejs.org" target="_blank" rel="noopener"
        >vue-cli documentation</a
      >.
    </p>

    <div class="p-d-flex">
      <DataTable :value="series" responsiveLayout="scroll">
        <template #header>
          <div class="table-header">Series</div>
        </template>
        <Column field="id" header="id"></Column>
        <Column field="name" header="Name"></Column>
        <Column header="Image">
          <template #body="slotProps">
            <img
              v-if="slotProps.data.image !== null"
              :src="slotProps.data.image.medium"
              class="product-image"
            />
          </template>
        </Column>
      </DataTable>
    </div>

    <img :src="pokemon.image" />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import seriesService from "@/services/series.service";
import pokemonService from "@/services/pokemon.service";
import playersService from "@/services/players.service";
import PokemonData from "@/services/pokemon.service";
import Players from "@/services/players.service";
import { ref, onMounted } from "vue";

export default defineComponent({
  name: "HelloWorld",
  props: {
    msg: String,
  },
  setup() {
    const pokemon_name = "pikachu";
    const series = ref([] as unknown);
    const pokemon = ref(PokemonData);
    const players = ref(Players);

    onMounted(() => {
      pokemonService.fetchPokemon(pokemon_name).then((p) => {
        console.log(p);
        pokemon.value = p;
      });

      seriesService.getSeries().then((response) => {
        let s = response.map((item) => item.show);
        console.log(s);
        series.value = s;
      });

      playersService.fetchPlayers().then(
        (p) => {
          console.log(p);
        },
        function (error) {
          console.log("Error:");
          console.log(error);
        }
      );
    });

    return { series, pokemon, players };
  },
});
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}

.table-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.hello {
  width: 90%;
  margin-left: auto;
  margin-right: auto;
}

.product-image {
  width: 100px;
  box-shadow: 0 3px 6px rgba(0, 0, 0, 0.16), 0 3px 6px rgba(0, 0, 0, 0.23);
}
</style>
