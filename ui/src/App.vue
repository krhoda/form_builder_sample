<template>
  <v-app>
    <v-main>
      <div v-if="targetForm">
        <FormViewer />
      </div>
      <div v-else>
        <Container />
      </div>
    </v-main>
  </v-app>
</template>

<script>
import Container from "./components/Container";
import FormViewer from "./components/FormViewer";

export default {
  name: "App",

  components: {
    Container,
    FormViewer,
  },

  created: function() {
    const params = new URLSearchParams(window.location.search);
    const target = params.get("form");
    if (target) {
      this.targetForm = target;
    }
  },

  data: () => ({
    targetForm: "",
  }),
};
</script>
