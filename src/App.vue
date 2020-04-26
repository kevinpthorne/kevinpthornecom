<template>
  <div class="main">
    <Billboard/>
    <AboutMe/>
  </div>
</template>

<script>

import Billboard from "@/components/Billboard";
import AboutMe from "@/components/AboutMe";

export default {
  name: 'App',
  components: {
    AboutMe,
    Billboard
  }
}
</script>

<style>

  .main {
    margin:0;
    padding:0;
    background-color: black;
  }

</style>
