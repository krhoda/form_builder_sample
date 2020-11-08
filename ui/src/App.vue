<template>
  <v-app>
    <v-main>
      <Container/>
    </v-main>
  </v-app>
</template>

<script>
import Container from './components/Container';

export default {
  name: 'App',

  components: {
    Container,
  },

  data: () => ({
    //
  }),
};
</script>
