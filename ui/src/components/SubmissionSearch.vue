<template>
  <v-container>
    <div v-if="!ready">
      <p>{{ message }}</p>
    </div>
    <div v-else>
      <v-card elevation="2">
        <v-card-text>
          <v-simple-table>
            <thead>
              <th>
                Form Name
              </th>
              <th>
                Form ID
              </th>
              <th>
                Submission ID
              </th>
            </thead>
            <tbody>
              <tr
                v-for="submission in submissionList"
                v-bind:key="`submission-${submission.submission_id}`"
              >
                <td>
                  <strong>{{ submission.form_name }}</strong>
                </td>
                <td>
                  <strong>{{ submission.form_id }}</strong>
                </td>
                <td>
                  <strong>{{ submission.submission_id }}</strong>
                </td>
              </tr>
            </tbody>
          </v-simple-table>
        </v-card-text>
      </v-card>
    </div>
  </v-container>
</template>

<script>
export default {
  name: "SubmissionSearch",

  created: function() {
    fetch(`http://${window.location.host}/list-submissions`)
      .then((res) => {
        if (res.ok && res.status === 200) {
          return res.json();
        }

        this.message = `Bad result in List Forms, response statusText: ${res.statusText}`;
        return false;
      })
      .then((json) => {
        if (json && json.submission_list) {
          this.submissionList = json.submission_list;
          this.ready = true;
          this.message = "";
        }
      })
      .catch((err) => {
        this.message = `Bad result in List Forms, err: ${err}`;
      });
  },

  data: () => ({
    message: "Loading...",
    ready: false,
    submissionList: false,
  }),
};
</script>
