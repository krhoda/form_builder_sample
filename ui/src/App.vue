<template>
  <v-app>
    <v-main>
      <div v-if="targetForm">
        <FormViewer />
      </div>
      <div v-else-if="formID && submissionID">
        <SubmissionViewer />
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
import SubmissionViewer from "./components/SubmissionViewer";

export default {
  name: "App",

  components: {
    Container,
    FormViewer,
    SubmissionViewer,
  },

  created: function() {
    const params = new URLSearchParams(window.location.search);
    const targetForm = params.get("form");
    if (targetForm) {
      this.targetForm = targetForm;
      return;
    }

    const formID = params.get("formID");
    const submissionID = params.get("submissionID");
    if (formID && submissionID) {
      this.submissionID = submissionID;
      this.formID = formID;
      return;
    }
  },

  data: () => ({
    targetForm: "",
    formID: "",
    submissionID: ""
  }),
};
</script>
