<template>
  <div>
    <div v-if="shareID !== ''">
      <v-card elevation="2">
        <v-card-title>The form ID is: {{ shareID }}</v-card-title>
        <v-card-text>
          To view it now:
          <a
            target="_blank"
            >click here</a
          >
        </v-card-text>
        <v-card-actions>
          <v-btn color="primary" v-on:click="backToForm"
            >Make Another Form</v-btn
          >
        </v-card-actions>
      </v-card>
    </div>
    <div v-if="shareID === ''">
      <v-card elevation="2">
        <v-card-title>
          Form Builder
        </v-card-title>
        <v-card-text>
          <p>Set the label, set the type, add it to the form!</p>
          <v-text-field label="Field's Label" v-model="label" />
          <v-radio-group v-model="input_type">
            <template v-slot:label>
              <div>Set the type</div>
            </template>
            <v-radio value="text">
              <template v-slot:label>
                <div>
                  String
                </div>
              </template>
            </v-radio>
            <v-radio value="number">
              <template v-slot:label>
                <div>
                  Number
                </div>
              </template>
            </v-radio>
          </v-radio-group>
        </v-card-text>
        <v-card-actions>
          <v-btn v-on:click="addToSchema" color="primary">+ Add to Form</v-btn>
        </v-card-actions>
      </v-card>
      <v-card elevation="2" class="mt-10">
        <v-card-title>The form will look like:</v-card-title>
        <v-card-text>
          <div v-if="Object.keys(schema).length === 0">
            None.
          </div>
          <div v-for="entry in schema" v-bind:key="`field-${entry.label}`">
            <v-text-field
              v-if="entry.input_type === 'text'"
              prepend-icon="mdi-format-text"
              :label="entry.label"
            ></v-text-field>
            <v-text-field
              v-else
              prepend-icon="mdi-numeric"
              :label="entry.label"
              type="number"
            ></v-text-field>
            <v-btn
              color="error"
              x-small
              v-on:click="deleteFromSchema(entry.label)"
              >Remove {{ entry.label }}</v-btn
            >
          </div>
          {{ JSON.stringify(schema) }}
        </v-card-text>
        <v-card-actions>
          <v-btn color="success" v-on:click="createForm">
            Save Form & Generate Link
          </v-btn>
        </v-card-actions>
      </v-card>
    </div>
  </div>
</template>

<script>
export default {
  name: "FormBuilder",

  data: () => {
    return {
      schema: {},
      label: "",
      input_type: "",
      shareID: "",
    };
  },

  methods: {
    addToSchema: function() {
      let valid = this.input_type !== "" && this.label !== "";

      if (!valid) {
        alert("Label and type are required!");
        return;
      }

      let dup = this.schema[this.label];
      if (dup) {
        alert(`Field ${this.label} has already been added!`);
        return;
      }

      this.schema[this.label] = {
        input_type: this.input_type,
        label: this.label,
      };

      this.input_type = "";
      this.label = "";
    },

    deleteFromSchema: function(label) {
      let nextSchema = Object.assign({}, this.schema);
      delete nextSchema[label];
      this.schema = nextSchema;
    },

    createForm: function() {
      fetch("http://localhost:8000/create-form", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ schema: this.schema }),
      })
      .then((res) => {
        if (res.ok && res.status === 200) {
          return res.json()
        } 

        alert(`Bad result in Form Submit, non-200 status code: ${res.status}`);
        return false;
      })
      .then((json) => {
        if (json) {
          this.shareID = json.form_id;
        }
      })
      .catch((err) => {
        alert(`Bad result in Form Submit: ${err}`);
      })
    },

    backToForm: function() {
      this.schema = {};
      this.shareID = "";
      this.input_type = "";
      this.label = "";
    },
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped></style>
