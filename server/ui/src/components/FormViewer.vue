<template>
  <v-container>
    <div v-if="!ready">
      <p>{{ message }}</p>
    </div>
    <div v-else-if="submissionID !== ''">
      <v-card elevation="2">
        <v-card-title>
          Your Submission has been saved!
        </v-card-title>
        <v-card-text>
          <p>Your submission ID is {{ submissionID }}</p>
          <p>The form ID is {{ targetForm }} </p>
        </v-card-text>
      </v-card>
    </div>
    <div v-else>
      <v-card elevation="2">
        <v-card-title>{{ schemaName }}</v-card-title>
        <v-card-text>
          {{ message }}
          <div v-for="entry in schema" v-bind:key="`field-${entry.label}`">
            <v-text-field
              v-if="entry.input_type === 'text'"
              prepend-icon="mdi-format-text"
              v-model="entry.value"
              :label="entry.label"
            ></v-text-field>
            <v-text-field
              v-else
              prepend-icon="mdi-numeric"
              v-model="entry.value"
              :label="entry.label"
              type="number"
            ></v-text-field>
          </div>
          <v-btn color="success" v-on:click="submitForm">Submit Form</v-btn>
        </v-card-text>
      </v-card>
    </div>
  </v-container>
</template>

<script>
export default {
  name: "FormViewer",

  created: function() {
    const params = new URLSearchParams(window.location.search);
    const target = params.get("form");
    if (!target) {
      this.message = "Could not find target form in query parameters";
      return;
    }

    this.targetForm = target;
    fetch(`http://${window.location.host}/fetch-form`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ form_id: this.targetForm }),
    })
      .then((res) => {
        if (res.ok && res.status === 200) {
          return res.json();
        }

        this.message = `Bad result in Get Form, response statusText: ${res.statusText}`;
        return false;
      })
      .then((json) => {
        if (json && json.contents) {
          let keys = Object.keys(json.contents.schema);
          let finalSchema = {};

          for (let i = 0, x = keys.length; i < x; i++) {
            let key = keys[i];
            let value = json.contents.schema[key];
            let next = Object.assign({ value: "" }, value);
            finalSchema[key] = next;
          }

          this.schema = finalSchema;
          this.schemaName = json.contents.schema_name;
          this.ready = true;
          this.message = "";
        }
      })
      .catch((err) => {
        this.message = `Bad result in Get Form, err: ${err}`;
      });
  },

  data: () => ({
    targetForm: "",
    schema: {},
    schemaName: "",
    message: "Loading...",
    ready: false,
    submissionID: "",
  }),

  methods: {
    submitForm: function() {
      fetch(`http://${window.location.host}/submit`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ submission: this.schema, form_id: this.targetForm }),
      }).then((res) => {
        if (res.ok && res.status === 200) {
          return res.json();
        }

        this.message = `Bad result in Submit Form, response statusText: ${res.statusText}`;
        return false;
      }).then((json) => {
        if (json && json.submission_id) {
          this.submissionID = json.submission_id;
        }
      }).catch((err) => {
        this.message = `Bad result in Submit Form, err: ${err}`;
      });
    },
  },
};
</script>
