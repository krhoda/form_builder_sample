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
            </thead>
            <tbody>
              <tr v-for="form in formList" v-bind:key="`form-${form.form_id}`">
                <td>
                  {{ form.form_name }}
                </td>
                <td>
                  {{ form.form_id }}
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
  name: "FormSearch",

  created: function() {
    fetch(`http://${window.location.host}/list-forms`)
      .then((res) => {
        if (res.ok && res.status === 200) {
          return res.json();
        }

        this.message = `Bad result in List Forms, response statusText: ${res.statusText}`;
        return false;
      })
      .then((json) => {
        console.log("HERE");
        if (json && json.form_list) {
          this.formList = json.form_list;
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
    formList: false,
  }),
};
</script>
