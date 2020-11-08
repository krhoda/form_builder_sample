<template>
  <v-container>
    <v-row class="text-center">
      <v-col cols="12">
        <v-row align="center" justify="space-around">
          <v-btn elevation="2" v-on:click="updateNav('builder')">
            Form Builder
          </v-btn>

          <v-btn elevation="2" v-on:click="updateNav('file-search')">
            Form Finder
          </v-btn>

          <v-btn elevation="2" v-on:click="updateNav('submission-search')">
            Submission Search
          </v-btn>
        </v-row>
      </v-col>

      <v-col class="mb-4">
        <div v-if="currentNav === 'none'">
          <p>Select a page</p>
        </div>

        <div v-if="currentNav === 'builder'">
          <FormBuilder/>
        </div>

        <div v-if="currentNav === 'file-search'">
          <p>Will be a form finder</p>
        </div>

        <div v-if="currentNav === 'submission-search'">
          <p>Will be a submission search</p>
        </div>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>
import FormBuilder from "./FormBuilder";
export default {
  name: "Container",
  components: {
    FormBuilder,
  },

  data: () => ({
    // host: window.location.host,
    currentNav: "none",
  }),

  methods: {
    updateNav: function(next) {
      this.currentNav = next;
    },
  },
};
</script>
