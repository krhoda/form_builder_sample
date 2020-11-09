<template>
  <v-container>
    <div v-if="!ready">
      <p>{{ message }}</p>
    </div>
    <div v-else>
      <v-card elevation="2">
        <v-card-title>{{ formName }}</v-card-title>
        <v-card-text>
          <div v-for="entry in submission" v-bind:key="`field-${entry.label}`">
            <p>
              <strong>{{ entry.label }}:</strong> {{ entry.value }}
            </p>
          </div>
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
    const formID = params.get("formID");
    const submissionID = params.get("submissionID");

    if (!formID) {
      this.message = "Could not find target form in query parameters";
      return;
    }

    if (!submissionID) {
      this.message = "Could not find target submission in query parameters";
      return;
    }

    this.formID = formID;
    this.submissionID = submissionID;

    fetch(`http://${window.location.host}/fetch-submission`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        form_id: this.formID,
        submission_id: this.submissionID,
      }),
    })
      .then((res) => {
        if (res.ok && res.status === 200) {
          return res.json();
        }

        this.message = `Bad result in Get Form, response statusText: ${res.statusText}`;
        return false;
      })
      .then((json) => {
        if (json && json.contents.submission) {
          this.submission = json.contents.submission;
          this.formName = json.contents.schema_name;
          this.ready = true;
          this.message = "";
        }
      })
      .catch((err) => {
        this.message = `Bad result in Get Form, err: ${err}`;
      });
  },

  data: () => ({
    message: "Loading...",
    ready: false,
    submission: false,
    formName: "",
  }),
};
</script>
