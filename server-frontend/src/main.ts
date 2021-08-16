import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import PrimeVue from "primevue/config";
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import Button from "primevue/button";

import "primevue/resources/themes/saga-blue/theme.css";
import "primevue/resources/primevue.min.css";
import "primeicons/primeicons.css";

const app = createApp(App);

app.use(PrimeVue);
app.use(router);
app.component("DataTable", DataTable);
app.component("Column", Column);
app.component("Button", Button);

app.mount("#app");
