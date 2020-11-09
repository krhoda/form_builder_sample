<template>
  <v-container>
    <v-row class="text-center">
      <v-col cols="12">
        <v-row align="center" justify="space-around">
          <v-btn elevation="2" v-on:click="updateNav('builder')">
            Form Builder
          </v-btn>

          <v-btn elevation="2" v-on:click="updateNav('file-search')">
            Form Table
          </v-btn>

          <v-btn elevation="2" v-on:click="updateNav('submission-search')">
            Submission Table
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
          <FormSearch />
        </div>

        <div v-if="currentNav === 'submission-search'">
          <SubmissionSearch />
        </div>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>
import FormBuilder from "./FormBuilder";
import FormSearch from "./FormSearch";
import SubmissionSearch from "./SubmissionSearch";

export default {
  name: "Container",
  components: {
    FormBuilder,
    FormSearch,
    SubmissionSearch,
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
