<template>
  <div>
    <v-row
      align="center"
      justify="space-around"
    >
        <v-btn elevation="2" :href="`${host}/form-builder`" >Form Builder</v-btn>
        <v-btn elevation="2" :href="`${host}/form-finder`" >Form Finder</v-btn>
        <v-btn elevation="2" :href="`${host}/sumbission-finder`">Submission Builder</v-btn>
    </v-row>
  </div>
</template>

<script>
export default {
  name: 'Nav',
  props: {
    navsetter: Function,
  },

  data: () => {
    return {
      host: window.location.host,
    }
  },
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>

</style>
