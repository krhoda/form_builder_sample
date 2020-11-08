<template>
  <div>
    <div class="hello">
      <a :href="`${host}/form-builder`" >Form Builder</a>
      <a :href="`${host}/form-finder`" >Form Finder</a>
      <a :href="`${host}/sumbission-finder`" >Submission Builder</a>
    </div>
  </div>
</template>

<script>
export default {
  name: 'Nav',
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
